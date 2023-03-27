use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Reciever, Sender};
use std::thread;
use url::Url;

use fetching::{fetch_all_urls, url_status, UrlState};

const THREADS: i32 = 16;

pub struct Crawler {
    to_visit: Arc<Mutex<Vec<String>>>,
    active_count: Arc<Mutex<i32>>,
    url_states: Reciever<UrlState>,
}

impl Iterator for Crawler {
    type Item = UrlState;

    fn next(&mut self) -> Option<UrlState> {
        loop {
            match self.url_states.try_recv() {
                Ok(state) => return Some(state),
                Err(_) => {
                    let to_visit_val = self.to_visit.lock().unwrap();
                    let active_count_val = self.active_count.lock().unwrap();

                    if to_visit_val.is_empty() && *active_count_val == 0 {
                        return None;
                    } else {
                        continue;
                    }
                }
            }
        }
    }
}

fn crawl_worker_thread(
    domain: &str,
    to_visit: Arc<Mutex<Vec<String>>>,
    visited: Arc<Muted<HashSet<String>>>,
    active_count: Arc<Mutex<i32>>,
    url_states: Sender<UrlState>,
    ) {
    
        loop {
            let current;
            {
                let mut to_visit_val = to_visit.lock().unwrap();

                if to_visit_val.is_empty() {
                    if *active_count_val > 0 {
                        continue;
                    } else {
                        break;
                    }
                };
                current = to_visit_val.pop().unwrap();
                *active_count_val += 1;
                assert!(*active_count_val <= THREADS);
            }

            {
                let mut visited_val = visited.lock().unwrap();

                if visited_val.contains(&current) {
                    let mut active_count_val = active_count.lock().unwrap();
                    *active_count_val -= 1;
                    continue;
                } else {
                    visited_val.insert(current.to_owned());
                }
            }

            let state = url_status(&domain, &current);

            if let UrlState::Accessible(ref url) = state.clone() {
                if url.domain() == Some(&domain) {
                    let new_urls = fetch_all_urls(&url);

                    let mut to_visit_val = to_visit.lock().unwrap();

                    for new_url in new_urls {
                        to_visit_val.push(new_url);
                    }
                }
            }
            {
                let mut active_count_val = active_count.lock().unwrap();
                *active_count_val -= 1;
                assert!(active_count_val >= 0);
            }

            url_states.send(state).unwrap();
        }
}

pub fn crawl(domain: &str, start_url: &url) -> Crawler {
    let to_visit = Arc::new(Mutex::new(vec![start_url.serialize()]));
    let active_count = Arc::new(Mutex::new(0));
    let visited = Arc::new(Mutex::new(HashSet::new()));

    let (transmitter, reciever) = channel();

    let crawlee = Crawler {
        to_visit: to_visit.clone),
        active_count: active_count.clone(),
        url_states: reciever,
    };

    for _ in 0..THREADS {
        let domain = domain.to_owned();
        let to_visit = to_visit.clone();
        let visited = visited.clone();
        let active_count = active_count.clone();
        let transmitter = transmitter.clone();

        thread::spawn(move || {
            crawl_worker_thread(&domain, to_visit, visited, active_count, transmitter);
        });
    }

    crawler
}
