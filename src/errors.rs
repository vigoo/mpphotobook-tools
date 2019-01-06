use error_chain::*;

error_chain! {
    errors {
        InvalidPath
        DecodeError
        MissingData
    }

    foreign_links {
        IO(::std::io::Error);
        ParseInt(::std::num::ParseIntError);
        ParseFloat(::std::num::ParseFloatError);
    }
}
