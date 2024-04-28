


#[cfg(test)]
mod tests {

use mini_redis::BlockingClient;
#[test]
fn test_blocking_client_set_get() {

        let mut client = BlockingClient::connect("127.0.0.1:6379").unwrap();

        let key = "test_key";
        let value = "test_value";

        // Set key-value pair
        let set_result = client.set(key, value.into());
        assert!(set_result.is_ok());

        // Get value for the key
        let get_result = client.get(key);
        assert!(get_result.is_ok());

        // Verify the retrieved value
        let retrieved_value = get_result.unwrap();
        assert_eq!(retrieved_value, Some(value.into()));
    }
    
}