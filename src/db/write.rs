use super::commit;
use crate::dag;
use crate::prolly;
pub struct Write<'a> {
    dag_write: &'a mut dag::Write<'a>,
    map: prolly::Map,
    basis_hash: &'a str,
}

#[allow(dead_code)]
impl<'a> Write<'a> {
    pub async fn new(
        basis_hash: &'a str,
        dag_write: &'a mut dag::Write<'a>,
    ) -> Result<Write<'a>, NewError> {
        let map = prolly::Map::load(basis_hash, dag_write.read()).await?;
        Ok(Write {
            basis_hash,
            dag_write,
            map,
        })
    }

    // TODO: This should move to a read struct, similar to dag::Write.
    pub fn get(&self, key: &[u8]) -> Option<&[u8]> {
        self.map.get(key)
    }

    pub fn put(&mut self, key: Vec<u8>, val: Vec<u8>) {
        self.map.put(key, val)
    }

    pub async fn commit(
        mut self,
        head_name: &str,
        local_create_date: &str,
        checksum: &str,
        mutation_id: u64,
        mutator_name: &str,
        mutator_args_json: &[u8],
        original_hash: Option<&str>,
    ) -> Result<(), CommitError> {
        let value_hash = self.map.flush(self.dag_write).await?;
        let commit = commit::Commit::new_local(
            local_create_date,
            self.basis_hash,
            checksum,
            mutation_id,
            mutator_name,
            mutator_args_json,
            original_hash,
            &value_hash,
        );

        let chunk = dag::Chunk::new(commit.take(), &[value_hash.as_str()][..]);
        self.dag_write.put_chunk(&chunk).await?;
        self.dag_write.set_head(head_name, chunk.hash()).await?;

        Ok(())
    }
}

// TODO: Find a way to mechanise below.
#[derive(Debug)]
pub enum NewError {
    Dag(dag::Error),
    MapLoad(prolly::LoadError),
}
impl From<dag::Error> for NewError {
    fn from(e: dag::Error) -> NewError {
        NewError::Dag(e)
    }
}
impl From<prolly::LoadError> for NewError {
    fn from(e: prolly::LoadError) -> NewError {
        NewError::MapLoad(e)
    }
}

#[derive(Debug)]
pub enum CommitError {
    Dag(dag::Error),
    Flush(prolly::map::FlushError),
}
impl From<dag::Error> for CommitError {
    fn from(e: dag::Error) -> CommitError {
        CommitError::Dag(e)
    }
}
impl From<prolly::FlushError> for CommitError {
    fn from(e: prolly::FlushError) -> CommitError {
        CommitError::Flush(e)
    }
}
