pub mod client;

pub mod network;

mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {
            // ::outermost::middle_secret_function(); // error, edition 2015에서만 유효한 문법
            crate::outermost::middle_secret_function(); // edition 2018에서 유효한 문법
        }

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    // outermost::middle_secret_function(); // error
    // outermost::inside::inner_function(); // error
    // outermost::inside::secret_function(); // error
}


#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}