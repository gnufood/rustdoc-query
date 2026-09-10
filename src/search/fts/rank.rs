//! Immutable lexical features used by the documentation-query ranker.

use std::collections::{BTreeSet, HashMap};

use tokio_rusqlite::rusqlite;

use super::index::IndexRow;
use super::tokenizer::{document_terms, query_terms};

#[derive(Clone, Debug)]
pub(super) struct RankFeatures {
    vocabulary: HashMap<String, u32>,
    items: HashMap<i64, ItemFeatures>,
}

#[derive(Clone, Debug)]
struct ItemFeatures {
    whole_name: String,
    identifier_terms: Vec<u32>,
    name_terms: Vec<u32>,
    documentation_terms: Vec<u32>,
}

impl RankFeatures {
    #[cfg(test)]
    pub(super) fn empty() -> Self {
        Self {
            vocabulary: HashMap::new(),
            items: HashMap::new(),
        }
    }

    pub(super) fn build(rows: &[IndexRow]) -> Self {
        let raw: Vec<_> = rows
            .iter()
            .map(|row| {
                let name_terms = unique_terms(document_terms(&row.name));
                let documentation_terms = unique_terms(document_terms(&row.docs));
                let identifier_terms = unique_terms(query_terms(&row.name));
                let whole_name = identifier_terms.first().cloned().unwrap_or_default();
                (
                    whole_name,
                    identifier_terms,
                    name_terms,
                    documentation_terms,
                )
            })
            .collect();
        let vocabulary: BTreeSet<_> = raw
            .iter()
            .flat_map(|(_, identifier, name, docs)| identifier.iter().chain(name).chain(docs))
            .cloned()
            .collect();
        let vocabulary: HashMap<_, _> = vocabulary
            .into_iter()
            .enumerate()
            .map(|(id, term)| (term, u32::try_from(id).unwrap_or(u32::MAX)))
            .collect();
        let items = raw
            .into_iter()
            .enumerate()
            .map(
                |(index, (whole_name, identifier_terms, name_terms, documentation_terms))| {
                    let rowid = i64::try_from(index + 1).unwrap_or(i64::MAX);
                    (
                        rowid,
                        ItemFeatures {
                            whole_name,
                            identifier_terms: ids(&vocabulary, identifier_terms),
                            name_terms: ids(&vocabulary, name_terms),
                            documentation_terms: ids(&vocabulary, documentation_terms),
                        },
                    )
                },
            )
            .collect();
        Self { vocabulary, items }
    }

    pub(super) fn persist(&self, tx: &rusqlite::Transaction<'_>) -> rusqlite::Result<()> {
        let mut terms = tx.prepare("INSERT INTO rank_terms(term_id, term) VALUES (?1, ?2)")?;
        let mut vocabulary: Vec<_> = self.vocabulary.iter().collect();
        vocabulary.sort_unstable_by_key(|(_, id)| **id);
        for (term, id) in vocabulary {
            terms.execute(rusqlite::params![id, term])?;
        }
        let mut items = tx.prepare(
            "INSERT INTO rank_features(rowid, whole_name, identifier_terms, name_terms, documentation_terms) \
             VALUES (?1, ?2, ?3, ?4, ?5)",
        )?;
        for (rowid, features) in &self.items {
            items.execute(rusqlite::params![
                rowid,
                features.whole_name,
                encode(&features.identifier_terms),
                encode(&features.name_terms),
                encode(&features.documentation_terms),
            ])?;
        }
        Ok(())
    }

    pub(super) fn load(conn: &rusqlite::Connection) -> rusqlite::Result<Self> {
        let mut terms = conn.prepare("SELECT term_id, term FROM rank_terms")?;
        let vocabulary = terms
            .query_map([], |row| {
                Ok((row.get::<_, String>(1)?, row.get::<_, u32>(0)?))
            })?
            .collect::<rusqlite::Result<HashMap<_, _>>>()?;
        let mut items = conn.prepare(
            "SELECT rowid, whole_name, identifier_terms, name_terms, documentation_terms FROM rank_features",
        )?;
        let items = items
            .query_map([], |row| {
                Ok((
                    row.get(0)?,
                    ItemFeatures {
                        whole_name: row.get(1)?,
                        identifier_terms: decode(&row.get::<_, Vec<u8>>(2)?)?,
                        name_terms: decode(&row.get::<_, Vec<u8>>(3)?)?,
                        documentation_terms: decode(&row.get::<_, Vec<u8>>(4)?)?,
                    },
                ))
            })?
            .collect::<rusqlite::Result<HashMap<_, _>>>()?;
        Ok(Self { vocabulary, items })
    }

    pub(super) fn exact_identifier(&self, rowid: i64, joined: &str) -> bool {
        self.items
            .get(&rowid)
            .is_some_and(|features| features.whole_name == joined)
    }

    pub(super) fn name_coverage(&self, rowid: i64, terms: &[String]) -> u16 {
        self.coverage(rowid, terms, |features| &features.name_terms)
    }

    pub(super) fn identifier_coverage(&self, rowid: i64, terms: &[String]) -> u16 {
        self.coverage(rowid, terms, |features| &features.identifier_terms)
    }

    pub(super) fn documentation_coverage(&self, rowid: i64, terms: &[String]) -> u16 {
        self.coverage(rowid, terms, |features| &features.documentation_terms)
    }

    fn coverage(
        &self,
        rowid: i64,
        terms: &[String],
        select: impl FnOnce(&ItemFeatures) -> &Vec<u32>,
    ) -> u16 {
        let Some(features) = self.items.get(&rowid) else {
            return 0;
        };
        let ids: Vec<_> = terms
            .iter()
            .filter_map(|term| self.vocabulary.get(term))
            .copied()
            .collect();
        let indexed = select(features);
        u16::try_from(
            ids.iter()
                .filter(|id| indexed.binary_search(id).is_ok())
                .count(),
        )
        .unwrap_or(u16::MAX)
    }
}

fn unique_terms(mut terms: Vec<String>) -> Vec<String> {
    terms.sort_unstable();
    terms.dedup();
    terms
}

fn ids(vocabulary: &HashMap<String, u32>, terms: Vec<String>) -> Vec<u32> {
    terms
        .into_iter()
        .filter_map(|term| vocabulary.get(&term).copied())
        .collect()
}

fn encode(ids: &[u32]) -> Vec<u8> {
    ids.iter().flat_map(|id| id.to_le_bytes()).collect()
}

fn decode(bytes: &[u8]) -> rusqlite::Result<Vec<u32>> {
    let (chunks, remainder) = bytes.as_chunks::<{ std::mem::size_of::<u32>() }>();
    if !remainder.is_empty() {
        return Err(rusqlite::Error::InvalidQuery);
    }
    Ok(chunks
        .iter()
        .map(|chunk| u32::from_le_bytes(*chunk))
        .collect())
}
