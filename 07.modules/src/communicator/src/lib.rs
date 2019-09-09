/// If a module named foo has no submodules, you should put the declarations for foo in a file named foo.rs.
/// If a module named foo does have submodules, you should put the declarations for foo in a file named foo/mod.rs.

mod network;
mod client;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
