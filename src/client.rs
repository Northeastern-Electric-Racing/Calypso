use crate::data::Data;

/**
 * Cummulative trait for structs that are able to connect to a server and publish data to it.
 */
pub trait Client {
    /**
     * Connects to the server at the given path.
     */
    fn connect(&mut self, path: &str) -> ();

    /**
     * Publishes the given data to the server.
     */
    fn publish(&mut self, data: &Data) -> ();
}
