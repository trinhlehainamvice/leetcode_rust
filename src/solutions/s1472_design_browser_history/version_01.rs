use std::cell::RefCell;
use std::rc::Rc;

struct UrlNode {
    url: String,
    next: Option<Rc<RefCell<UrlNode>>>,
    prev: Option<Rc<RefCell<UrlNode>>>,
}

struct BrowserHistory {
    current: Option<Rc<RefCell<UrlNode>>>,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {
    fn new(homepage: String) -> Self {
        BrowserHistory {
            current: Some(Rc::new(RefCell::new(UrlNode {
                url: homepage,
                next: None,
                prev: None,
            }))),
        }
    }

    fn visit(&mut self, url: String) {
        let url_node = Some(Rc::new(RefCell::new(UrlNode {
            url,
            next: None,
            prev: self.current.clone(),
        })));
        self.current.as_ref().unwrap().borrow_mut().next = url_node.clone();
        self.current = url_node;
    }

    fn back(&mut self, steps: i32) -> String {
        let mut current = self.current.clone();
        let mut steps = steps;

        loop {
            if steps == 0 {
                break;
            }

            match current.clone() {
                Some(node) => {
                    match node.borrow().prev.clone() {
                        Some(prev_node) => {
                            current = Some(prev_node);
                        },
                        None => {
                            break;
                        }
                    }
                },
                None => {
                    break;
                }
            }
            
            steps -= 1;

            if current.as_ref().unwrap().borrow().prev.is_none() {
                break;
            }
        }
        
        self.current = current.clone();
        
        current.unwrap().borrow().url.to_string()
    }

    fn forward(&mut self, steps: i32) -> String {
        let mut current = self.current.clone();
        let mut steps = steps;
        
        loop {
            if steps == 0 {
                break;
            }
            
            match current.clone() {
                Some(node) => {
                    match node.borrow().next.clone() {
                        Some(next_node) => {
                            current = Some(next_node);
                        }
                        None => {
                            break;
                        }
                    }
                }
                None => {
                    break;
                }
            }
            
            steps -= 1;
            
            if current.as_ref().unwrap().borrow().next.is_none() {
                break;
            }
        }
        
        self.current = current.clone();
        
        current.unwrap().borrow().url.to_string()
    }
}

/**
 * Your BrowserHistory object will be instantiated and called as such:
 * let obj = BrowserHistory::new(homepage);
 * obj.visit(url);
 * let ret_2: String = obj.back(steps);
 * let ret_3: String = obj.forward(steps);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut obj = BrowserHistory::new("leetcode.com".to_string()); 
        obj.visit("google.com".to_string());
        obj.visit("facebook.com".to_string());
        obj.visit("youtube.com".to_string());
        assert_eq!(obj.back(1), "facebook.com".to_string());
        assert_eq!(obj.back(1), "google.com".to_string());
        assert_eq!(obj.forward(1), "facebook.com".to_string());
        obj.visit("linkedin.com".to_string());
        assert_eq!(obj.forward(2), "linkedin.com".to_string());
        assert_eq!(obj.back(2), "google.com".to_string());
        assert_eq!(obj.back(7), "leetcode.com".to_string());
    }
}