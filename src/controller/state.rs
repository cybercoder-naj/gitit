pub struct GitState {
    unstaged_files: Vec<&'static str>,
    commit_msg: String,
    commit_desc: String,
    branch_name: String,

    listeners: Vec<&'static dyn Fn(&Self)>
}

impl GitState {
    pub fn new() -> Self {
        Self {
            unstaged_files: vec![],
            commit_msg: "".to_string(),
            commit_desc: "".to_string(),
            branch_name: "".to_string(),

            listeners: vec![]
        }
    }

    fn notify_observers(&self) {
        for f in &self.listeners {
            (*f)(self);
        }
    }

    pub fn observe(&mut self, f: &'static dyn Fn(&Self)) {
        self.listeners.push(f);
    }
}