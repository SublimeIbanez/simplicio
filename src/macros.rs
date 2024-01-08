

#[macro_export]
macro_rules! s {
    ($str:expr) => { String::from($str) };
}

#[macro_export]
macro_rules! cnct {
    ($e1:expr $(, $e2:expr)*) => {
        {
            let mut string: String = String::from($e1);
            $(
                let second: &str = &$e2;
                string.push_str(second);
            )+
            string
        }
    };
}
