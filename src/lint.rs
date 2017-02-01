extern crate xml;

use std::cmp;
use std::env;
use std::io::{self, Read, Write, BufReader};
use std::fs::File;
use std::collections::HashSet;
use std::borrow::Cow;

use xml::ParserConfig;
use xml::writer::EmitterConfig;
use xml::reader::XmlEvent;
use xml::writer::XmlEvent as WriterXmlEvent;
use xml::attribute::Attribute;

macro_rules! abort {
    ($code:expr) => {::std::process::exit($code)};
    ($code:expr, $($args:tt)+) => {{
        writeln!(&mut ::std::io::stderr(), $($args)+).unwrap();
        ::std::process::exit($code);
    }}
}

fn main() {
    let mut file;
    let mut stdin;
    let source: &mut Read = match env::args().nth(1) {
        Some(file_name) => {
            file = File::open(file_name)
                .unwrap_or_else(|e| abort!(1, "Cannot open input file: {}", e));
            &mut file
        }
        None => {
            stdin = io::stdin();
            &mut stdin
        }
    };

    let reader = ParserConfig::new()
        .whitespace_to_characters(true)
        .ignore_comments(false)
        .create_reader(BufReader::new(source));

    //    let mut target: Vec<u8> = Vec::new();
    let mut target = std::io::stdout();
    let mut writer = EmitterConfig::new()
        .line_separator("\n")
        .perform_indent(false)
        .normalize_empty_elements(false)
        .create_writer(&mut target);

    for e in reader {
        match e {
            Ok(e) => match e {
                XmlEvent::StartDocument { version, encoding, standalone } => {
                    writer.write(WriterXmlEvent::StartDocument {
                        version: version,
                        encoding: Some(&encoding),
                        standalone: standalone
                    });
                }
                XmlEvent::EndDocument => println!("Document finished"),
                XmlEvent::ProcessingInstruction { name, data } => {
                    let d: Option<&str> = data.as_ref().map(String::as_ref);
                    writer.write(WriterXmlEvent::ProcessingInstruction {
                        name: &name,
                        data: d
                    });
                }
                XmlEvent::Whitespace(s) => {
                    writer.write(WriterXmlEvent::Characters(&s));
                }
                XmlEvent::Characters(s) => {
                    writer.write(WriterXmlEvent::Characters(&s));
                }
                XmlEvent::CData(s) => {
                    writer.write(WriterXmlEvent::Characters(&s));
                }
                XmlEvent::Comment(s) => {
                    writer.write(WriterXmlEvent::Comment(&s));
                }
                XmlEvent::StartElement { name, attributes, namespace } => {
                    let attrs: Vec<Attribute> = attributes.iter().map(|a| a.borrow()).collect();
                    writer.write(WriterXmlEvent::StartElement {
                        name: name.borrow(),
                        attributes: Cow::Borrowed(&attrs),
                        namespace: Cow::Borrowed(&namespace)
                    });
                }
                XmlEvent::EndElement { name } => {
                    writer.write(WriterXmlEvent::EndElement {
                        name: Some(name.borrow())
                    });
                }
            },
            Err(e) => abort!(1, "Error parsing XML document: {}", e)
        }
    }
}
