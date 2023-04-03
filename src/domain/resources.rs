#[derive(Copy, Clone)]
pub struct ResourceLoader {
    binary_loader: fn(String) -> Vec<u8>,
    path_generator: fn(String) -> String,
}

impl ResourceLoader {
    pub fn new(binary_loader: fn(String) -> Vec<u8>, path_generator: fn(String) -> String) -> Self {
        Self {
            binary_loader,
            path_generator,
        }
    }

    pub fn load_as_binary(&self, path: String) -> Vec<u8> {
        (self.binary_loader)(path)
    }

    pub fn load_as_string(&self, path: String) -> String {
        let binary = (self.binary_loader)(path);
        String::from_utf8(binary).unwrap()
    }

    pub fn load_as_file(&self, path: String) -> String {
        (self.path_generator)(path)
    }
}

#[cfg(test)]
mod resource_loader_test {
    use spectral::prelude::*;

    use crate::domain::resources::ResourceLoader;

    #[test]
    fn should_return_binary_as_is() {
        let loader = ResourceLoader::new(|_| vec![1, 2, 3], |_| String::from("test"));

        let loaded = loader.load_as_binary(String::from("path"));

        assert_that!(loaded).is_equal_to(vec![1, 2, 3]);
    }

    #[test]
    fn should_return_string_as_utf8() {
        let loader = ResourceLoader::new(|_| String::from("éば~").into_bytes(), |_| String::from("test"));

        let loaded = loader.load_as_string(String::from("path"));

        assert_that!(loaded).is_equal_to(String::from("éば~"));
    }

    #[test]
    fn should_get_file_path() {
        let loader = ResourceLoader::new(|_| vec![], |_| String::from("/path"));

        let path = loader.load_as_file(String::from("path"));

        assert_that!(path).is_equal_to(String::from("/path"));
    }
}