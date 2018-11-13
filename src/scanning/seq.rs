pub struct CharacterSequence<'a, T: 'a + Copy> {
    previous_item: Option<T>,
    next_item: Option<T>,
    lookahead_item: Option<T>,
    source: &'a mut Iterator<Item=T>,
}

impl<'a, T: Copy> CharacterSequence<'a, T> {
    pub fn new(source: &'a mut Iterator<Item=T>) -> CharacterSequence<'a, T> {
        let mut s = CharacterSequence {
            previous_item: None,
            next_item: None,
            lookahead_item: None,
            source,
        };

        s.next();
        s.next();
        
        return s;
    }
    
    pub fn peek_next(&self) -> Option<T> {
        self.next_item
    }
    
    pub fn peek_ahead(&self) -> Option<T> {
        self.lookahead_item
    }
    
    pub fn previous(&self) -> Option<T> {
        self.previous_item
    }
}

impl<'a, T: Copy> Iterator for CharacterSequence<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let next_item = self.source.next();
        
        if self.next_item.is_some() {
            self.previous_item = self.next_item;
        }
        let result = self.next_item;
        self.next_item = self.lookahead_item;
        self.lookahead_item = next_item;

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_iterates_in_correct_order() {
        //+ arrange
        let collection = [1,2,3,4,5];
        let mut iterator = collection.iter();
        let mut sequence = CharacterSequence::new(&mut iterator);

        //+ act & assert
        assert_eq!(sequence.next(), Some(&1));
        assert_eq!(sequence.next(), Some(&2));
        assert_eq!(sequence.next(), Some(&3));
        assert_eq!(sequence.next(), Some(&4));
        assert_eq!(sequence.next(), Some(&5));
        assert_eq!(sequence.next(), None);
    }

    #[test]
    fn sequence_peek_next_shows_next_item_to_be_returned() {
        //+ arrange
        let collection = [1,2,3,4,5];
        let mut iterator = collection.iter();
        let mut sequence = CharacterSequence::new(&mut iterator);

        //+ arrange
        sequence.next();
        sequence.next();
        
        //+ assert
        assert_eq!(sequence.peek_next(), Some(&3));
        assert_eq!(sequence.next(), Some(&3));
    }

    #[test]
    fn sequence_peek_ahead_shows_item_after_next_to_be_returned() {
        //+ arrange
        let collection = [1,2,3,4,5];
        let mut iterator = collection.iter();
        let mut sequence = CharacterSequence::new(&mut iterator);

        //+ arrange
        sequence.next();
        sequence.next();
        
        //+ assert
        assert_eq!(sequence.peek_ahead(), Some(&4));
        assert_eq!(sequence.next(), Some(&3));
    }

    #[test]
    fn sequence_maintains_previous_item_when_iteration_complete() {
        //+ arrange
        let collection = [1,2,3,4,5];
        let mut iterator = collection.iter();
        let mut sequence = CharacterSequence::new(&mut iterator);

        //+ act & assert
        sequence.next();
        sequence.next();
        sequence.next();
        sequence.next();
        sequence.next();
        sequence.next();
        
        //+ assert
        assert_eq!(sequence.previous(), Some(&5));
    }
}