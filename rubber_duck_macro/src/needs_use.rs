fn parse_attrs(args: &mut ItemFn) -> Vec<ParsedAttr> {
  crate::util::drain_map(&mut args.attrs, |attr| {
    let meta = attr.interpret_meta();
    if let Some(meta) = meta {
      Some(parse_meta(&meta))
    } else {
      panic!("Not a parseable meta: {:?}", meta);
    }
  })
}

enum AttrLocation {
  Struct,
  Field(String),
}

struct ParsedAttr {
  location: AttrLocation,
  content: Meta,
}

fn parse_meta(meta: &Meta) -> ParsedAttr {
  let res: ParsedAttr = match meta {
    Meta::List(meta) => {
      if "for_field" == format!("{}", meta.ident) {
        let mut iter = meta.nested.iter();
        if let Some(first) = iter.next() {
          let field_name = match first {
            NestedMeta::Literal(first) => format!("{:?}", first),
            NestedMeta::Meta(first) => match first {
              Meta::Word(ident) => format!("{:?}", first),
              _ => panic!("Expected a string literal or single identifier"),
            },
          };
          if let Some(second) = iter.next() {
            let parsed = ParsedAttr {
              location: AttrLocation::Field(field_name),
              content: match second {
                NestedMeta::Literal(_) => panic!(
                  "Cannot use a string literal for the annotation: {:?}",
                  second
                ),
                NestedMeta::Meta(second) => second.clone(),
              },
            };
            if iter.next().is_some() {
              panic!("Too many items in the attribute list: {:?}", meta)
            }
            parsed
          } else {
            panic!("Not enough items in attribute list");
          }
        } else {
          panic!("Not enough values passed to 'for_field'")
        }
      } else if "for_struct" == format!("{}", meta.ident) {
        let mut iter = meta.nested.iter();
        if let Some(first) = iter.next() {
          let parsed = ParsedAttr {
            location: AttrLocation::Struct,
            content: match first {
              NestedMeta::Literal(_) => panic!(
                "Cannot use a string literal for the annotation: {:?}",
                first
              ),
              NestedMeta::Meta(nested_meta) => nested_meta.clone(),
            },
          };
          if iter.next().is_some() {
            panic!("Too many items in the attribute list: {:?}", &meta)
          } else {
            parsed
          }
        } else {
          panic!("Not enough items in the attribute list: {:?}", &meta)
        }
      } else {
        panic!("Unknown attribute! {:?}", &meta.ident);
      }
    }
    _ => panic!("Unknown field {:?}", &meta),
  };
  return res;
}