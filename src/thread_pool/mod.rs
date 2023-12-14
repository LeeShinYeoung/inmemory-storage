use std::{
  collections::VecDeque,
  sync::mpsc::{channel, sync_channel, Receiver, Sender},
  thread::{Builder, JoinHandle},
};

use crate::storage::size;

type Work = dyn FnOnce() + Send + 'static;

enum Message<T> {
  New(T),
  Term,
}

pub struct ThreadPool {
  ready: Box<Receiver<ThreadWorker>>,
  done: Box<Sender<Message<ThreadWorker>>>,
  main: Option<JoinHandle<()>>,
}

impl ThreadPool {
  pub fn new(max_len: usize, size: usize) -> Self {
    let (ready_s, ready_r) = sync_channel(max_len);
    let (done_s, done_r) = channel::<Message<ThreadWorker>>();

    for i in 0..max_len {
      let thread = ThreadWorker::new(size, i.to_string());
      ready_s.send(thread).unwrap();
    }

    let main = Builder::new()
      .name(String::from("main"))
      .stack_size(size::kb(size * 8))
      .spawn(move || {
        let ready_s = Box::new(ready_s);
        while let Message::New(mut worker) = done_r.recv().unwrap() {
          let ready_s = ready_s.clone();
          Builder::new()
            .name(worker.name.clone() + "_done")
            .stack_size(size::kb(4))
            .spawn(move || {
              while let Some(done) = worker.done.pop_front() {
                done.iter().for_each(drop);
              }
              ready_s.send(worker).unwrap();
            })
            .unwrap();
        }
      })
      .unwrap();

    Self {
      ready: Box::new(ready_r),
      done: Box::new(done_s),
      main: Some(main),
    }
  }

  pub fn schedule<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let mut w = self.ready.recv().unwrap();
    w.schedule(f);
    self.done.send(Message::New(w)).unwrap();
  }
}
impl Drop for ThreadPool {
  fn drop(&mut self) {
    self.done.send(Message::Term).unwrap();
    if let Some(main) = self.main.take() {
      main.join().unwrap();
    }
    for thread in self.ready.iter() {
      drop(thread);
    }
  }
}

struct ThreadWorker {
  channel: Box<Sender<Message<(Box<Work>, Sender<()>)>>>,
  done: Box<VecDeque<Receiver<()>>>,
  name: String,
  thread: Option<JoinHandle<()>>,
}

impl ThreadWorker {
  fn new(size: usize, name: String) -> Self {
    let (tx, rx) = channel::<Message<(Box<Work>, Sender<()>)>>();
    let thread = Builder::new()
      .stack_size(size)
      .name(name.to_owned())
      .spawn(move || {
        while let Message::New((job, done)) = rx.recv().unwrap() {
          job();
          done.send(()).unwrap();
        }
      })
      .unwrap();

    Self {
      channel: Box::new(tx),
      done: Box::new(VecDeque::new()),
      name,
      thread: Some(thread),
    }
  }

  fn schedule<F>(&mut self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let (tx, rx) = channel();
    let job = Box::new(f);
    self.channel.send(Message::New((job, tx))).unwrap();
    self.done.push_back(rx);
  }
}
impl Drop for ThreadWorker {
  fn drop(&mut self) {
    if let Some(t) = self.thread.take() {
      self.channel.send(Message::Term).unwrap();
      t.join()
        .expect(&("panic on thread ".to_owned() + &self.name));
    }
  }
}
