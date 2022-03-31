use crate::{escape::Escaper, ser::{Error, Result}};
use std::io::Write;

pub(super) trait WriteExt {
    fn write_tag_name_unescaped(&mut self, tag_name: &[u8]) -> Result<()>;
    fn write_tag_name_escaped(&mut self, tag_name: &[u8]) -> Result<()>;
    fn write_parameter_unescaped(&mut self, parameter: &[u8]) -> Result<()>;
    fn write_parameter_escaped(&mut self, parameter: &[u8]) -> Result<()>;
    fn close_tag(&mut self) -> Result<()>;
}

impl<W> WriteExt for W where W: Write {
    fn write_tag_name_unescaped(&mut self, tag_name: &[u8]) -> Result<()> {
        self.write_all(b"#").or(Err(Error::Io))?;
        self.write_all(tag_name).or(Err(Error::Io))
    }

    fn write_tag_name_escaped(&mut self, tag_name: &[u8]) -> Result<()> {
        self.write_tag_name_unescaped(&Escaper::new(tag_name).collect::<Vec<_>>())
    }

    fn write_parameter_unescaped(&mut self, parameter: &[u8]) -> Result<()> {
        self.write_all(b":").or(Err(Error::Io))?;
        self.write_all(parameter).or(Err(Error::Io))
    }

    fn write_parameter_escaped(&mut self, parameter: &[u8]) -> Result<()> {
        self.write_parameter_unescaped(&Escaper::new(parameter).collect::<Vec<_>>())
    }

    fn close_tag(&mut self) -> Result<()> {
        self.write_all(b";\n").or(Err(Error::Io))
    }
}

#[cfg(test)]
mod tests {
    use super::WriteExt;
    use claim::{assert_ok, assert_err};
    use std::{io, io::Write};

    #[test]
    fn write_tag_name_unescaped_regular() {
        let mut output = Vec::new();

        assert_ok!(output.write_tag_name_unescaped(b"foo"));

        assert_eq!(output, b"#foo");
    }

    #[test]
    fn write_tag_name_unescaped_escapes() {
        let mut output = Vec::new();

        assert_ok!(output.write_tag_name_unescaped(b"fo#o"));

        assert_eq!(output, b"#fo#o");
    }

    #[test]
    fn write_tag_name_escaped_regular() {
        let mut output = Vec::new();

        assert_ok!(output.write_tag_name_escaped(b"foo"));

        assert_eq!(output, b"#foo");
    }

    #[test]
    fn write_tag_name_escaped_escapes() {
        let mut output = Vec::new();

        assert_ok!(output.write_tag_name_escaped(b"fo#o"));

        assert_eq!(output, b"#fo\\#o");
    }

    #[test]
    fn write_parameter_unescaped_regular() {
        let mut output = Vec::new();

        assert_ok!(output.write_parameter_unescaped(b"foo"));

        assert_eq!(output, b":foo");
    }

    #[test]
    fn write_parameter_unescaped_escapes() {
        let mut output = Vec::new();

        assert_ok!(output.write_parameter_unescaped(b"fo#o"));

        assert_eq!(output, b":fo#o");
    }

    #[test]
    fn write_parameter_escaped_regular() {
        let mut output = Vec::new();

        assert_ok!(output.write_parameter_escaped(b"foo"));

        assert_eq!(output, b":foo");
    }

    #[test]
    fn write_parameter_escaped_escapes() {
        let mut output = Vec::new();

        assert_ok!(output.write_parameter_escaped(b"fo#o"));

        assert_eq!(output, b":fo\\#o");
    }

    #[test]
    fn close_tag() {
        let mut output = Vec::new();

        assert_ok!(output.close_tag());

        assert_eq!(output, b";\n");
    }

    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "failed"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::new(io::ErrorKind::Other, "failed"))
        }
    }

    #[test]
    fn write_tag_name_unescaped_failure() {
        let mut output = FailingWriter;

        assert_err!(output.write_tag_name_unescaped(b"foo"));
    }

    #[test]
    fn write_tag_name_escaped_failure() {
        let mut output = FailingWriter;

        assert_err!(output.write_tag_name_escaped(b"foo"));
    }

    #[test]
    fn write_parameter_unescaped_failure() {
        let mut output = FailingWriter;

        assert_err!(output.write_parameter_unescaped(b"foo"));
    }

    #[test]
    fn write_parameter_escaped_failure() {
        let mut output = FailingWriter;

        assert_err!(output.write_parameter_escaped(b"foo"));
    }

    #[test]
    fn close_tag_failure() {
        let mut output = FailingWriter;

        assert_err!(output.close_tag());
    }
}
