pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    use super::client;
    #[test]
    fn it_works() {
      // OK: 2015, NG: 2018
      ::client::connect();
      // OK: 2015, OK: 2018
      super::client::connect();
      // use super::client; => OK: 2015, OK: 2018
      client::connect();
    }
}
