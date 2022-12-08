use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::ops::Deref;
use std::collections::HashMap;

pub type DirRef = Rc<DirectoryContents>;
pub type WeakDirRef = Weak<DirectoryContents>;
pub type Parent = RefCell<WeakDirRef>;
pub type Children = RefCell<HashMap<String, Child>>;
pub type Child = DirRef;
pub type Files = RefCell<Vec<File>>;

#[derive(Debug, Clone)]
pub struct File {
    name: String,
    size: i64
}

impl File {
    pub fn new(name: &str, size: i64) -> Self {
        Self { name: name.to_string(), size }
    }
}


pub struct Directory {
    dir_ref: DirRef
}

impl Directory {
    pub fn new(name: &str) -> Self {
        let contents = DirectoryContents::new(name);

        let dir_ref = Rc::new(contents);
        Self { dir_ref }
    }

    pub fn get_ref(&self) -> DirRef {
        Rc::clone(&self.dir_ref)
    }
}

impl Deref for Directory {
    type Target = DirectoryContents;

    fn deref(&self) -> &Self::Target {
        &self.dir_ref
    }
}

#[derive(Debug, Clone)]
pub struct DirectoryContents {
    pub name: String,
    pub files: Files,
    pub parent: Parent,
    pub children: Children,
}

impl DirectoryContents {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(HashMap::new()),
        }
    }

    pub fn add_directory(&self, name: &str, parent: WeakDirRef) {
        let new_dir = Directory::new(name);
        self.children.borrow_mut().insert(name.to_string(), new_dir.get_ref());

        let mut new_dir_parent = new_dir.parent.borrow_mut();
        *new_dir_parent = parent;
    }

    pub fn add_file(&self, name: &str, size: i64) {
        self.files.borrow_mut().push(File::new(name, size));
    }

    pub fn get_child(&self, name: &str) -> DirRef {
        Rc::clone(self.children.borrow().get(name).unwrap())
    }

    pub fn get_parent(&self) -> Option<DirRef> {
        let weak = self.parent.borrow();
        if let Some(strong_ref) = weak.upgrade() {
            Some(strong_ref)
        } else {
            None
        }
    }

    pub fn size(&self) -> i64 {
        // Fix this
        0
    }
}