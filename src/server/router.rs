use crate::protocol::{DeleteRequest, Error, GetRequest, Request, Response, Result, SetRequest};
use crate::storage::{MaxMemoryStrategy, Storage};

pub struct RequestRouter {
  storage: Storage,
}

impl RequestRouter {
  pub fn new(storage_size: usize, max_memory_strategy: MaxMemoryStrategy) -> Self {
    Self {
      storage: Storage::new(storage_size, max_memory_strategy),
    }
  }

  pub fn handle(&mut self, request: Request) -> Result<Response> {
    match request {
      Request::Get(request) => self.get(request),
      Request::Set(request) => self.set(request),
      Request::Delete(request) => self.delete(request),
    }
  }

  fn get(&mut self, req: GetRequest) -> Result<Response> {
    self
      .storage
      .get(&req.key)
      .map(|result| Response::success(result))
      .map_err(Error::IO)
  }

  fn set(&mut self, req: SetRequest) -> Result<Response> {
    self
      .storage
      .put(req.key, req.value, req.ttl)
      .map(|_| Response::success(Default::default()))
      .map_err(Error::IO)
  }

  fn delete(&mut self, req: DeleteRequest) -> Result<Response> {
    self
      .storage
      .del(&req.key)
      .map(|_| Response::success(Default::default()))
      .map_err(Error::IO)
  }
}
