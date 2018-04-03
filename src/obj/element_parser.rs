use obj::object::{Element, VTNIndex};
use obj::parser::{ParserState, ParseError};


struct ElementParser {}

impl ElementParser {
    fn new() -> ElementParser { ElementParser {} } 

    fn parse_point(&self, 
        state: &mut ParserState, 
        elements: &mut Vec<Element>) -> Result<(), ParseError> {
        
        try!(state.expect("p"));
        let v_index = try!(state.parse_u32());
        elements.push(Element::Point(VTNIndex::new(v_index as usize, None, None)));
        loop {
            match state.next_string().as_ref().map(|st| &st[..]) {
                Ok("\n") | Err(_) => break,
                Ok(st) => match st.parse::<usize>() {
                    Ok(v_index) => elements.push(
                        Element::Point(VTNIndex::new(v_index, None, None))
                    ),
                    Err(_) => return state.error(format!("Expected integer but got `{}`.", st))
                }
            }
        }

        Ok(())
    }

    fn parse_line(&self, 
        state: &mut ParserState, 
        elements: &mut Vec<Element>) -> Result<(), ParseError> {
        
        try!(state.expect("l"));
        
        let vtn_index1 = try!(state.parse_u32());
        let vtn_index2 = try!(state.parse_u32());
        elements.push(Element::Line(
            VTNIndex::new(vtn_index1 as usize, None, None),
            VTNIndex::new(vtn_index2 as usize, None, None)
        ));
        let mut vtn_index1 = VTNIndex::new(vtn_index2 as usize, None, None);
        loop {
            match state.next_string().as_ref().map(|st| &st[..]) {
                Ok("\n") | Err(_) => break,
                Ok(st) => match st.parse::<usize>() {
                    Ok(v_index) => { 
                        let vtn_index2 = VTNIndex::new(v_index, None, None);
                        elements.push(Element::Line(vtn_index1, vtn_index2));
                        vtn_index1 = vtn_index2;
                    },
                    Err(_) => return state.error(format!("Expected integer but got `{}`.", st))
                }
            }
        }

        Ok(())     
    }

    fn parse_face(&self,
        state: &mut ParserState, 
        elements: &mut Vec<Element>) -> Result<(), ParseError> {
        
        unimplemented!();
    }

    fn parse(&self, 
        state: &mut ParserState,
        elements: &mut Vec<Element>) -> Result<(), ParseError> {
        
        match state.peek().as_ref().map(|st| &st[..]) {
            Some("p") => self.parse_point(state, elements),
            Some("l") => self.parse_line(state, elements),
            Some("f") => self.parse_face(state, elements),
            _ => unimplemented!(),
        }
    }
}


#[cfg(test)]
mod tests {
    use obj::object::{Element, VTNIndex};
    use obj::parser::ParserState;


    #[test]
    fn test_parse_point1() {
        let mut state = ParserState::new("p 1 2 3 4 \n");
        let parser = super::ElementParser::new();
        let mut result = Vec::new();
        parser.parse(&mut state, &mut result).unwrap();
        let expected = vec![
            Element::Point(VTNIndex::new(1, None, None)), Element::Point(VTNIndex::new(2, None, None)),
            Element::Point(VTNIndex::new(3, None, None)), Element::Point(VTNIndex::new(4, None, None)),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line1() {
        let mut state = ParserState::new("l 297 38 118 108 \n");
        let parser = super::ElementParser::new();
        let mut result = Vec::new();
        parser.parse(&mut state, &mut result).unwrap();
        let expected = vec![
            Element::Line(VTNIndex::new(297, None, None), VTNIndex::new(38,  None, None)), 
            Element::Line(VTNIndex::new(38,  None, None), VTNIndex::new(118, None, None)),
            Element::Line(VTNIndex::new(118, None, None), VTNIndex::new(108,   None, None)),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line2() {
        let mut state = ParserState::new("l 297/38 118/108 \n");
        let parser = super::ElementParser::new();
        let mut result = Vec::new();
        parser.parse(&mut state, &mut result).unwrap();
        let expected = vec![
            Element::Line(VTNIndex::new(297, Some(38), None), VTNIndex::new(118, Some(108), None)),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line3() {
        let mut state = ParserState::new("l 297/38 118/108 324/398 \n");
        let parser = super::ElementParser::new();
        let mut result = Vec::new();
        parser.parse(&mut state, &mut result).unwrap();
        let expected = vec![
            Element::Line(VTNIndex::new(297, Some(38),  None), VTNIndex::new(118, Some(108), None)),
            Element::Line(VTNIndex::new(118, Some(108), None), VTNIndex::new(324, Some(398), None)),
        ];
        assert_eq!(result, expected);
    }
}

