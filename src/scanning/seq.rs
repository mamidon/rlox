pub struct CharacterSequence<'a, T: 'a + Copy> {
    previous: Option<T>,
    current: Option<T>,
    next: Option<T>,
    source: &'a mut Iterator<Item=T>,
}

impl<'a, T: Copy> CharacterSequence<'a, T> {
    pub fn new(source: &'a mut Iterator<Item=T>) -> CharacterSequence<'a, T> {
        let mut s = CharacterSequence {
            previous: None,
            current: None,
            next: None,
            source,
        };

        s.next();

        return s;
    }
    
    pub fn peek(&self) -> Option<T> {
        self.next
    }
    
    pub fn previous(&self) -> Option<T> {
        self.previous
    }
}

impl<'a, T: Copy> Iterator for CharacterSequence<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let next_item = self.source.next();

        if self.current.is_some() {
            self.previous = self.current;
        }
        self.current = self.next;
        self.next = next_item;

        return self.current;
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
    fn sequence_maintains_one_lookahead() {
        //+ arrange
        let collection = [1,2,3,4,5];
        let mut iterator = collection.iter();
        let mut sequence = CharacterSequence::new(&mut iterator);

        //+ arrange
        sequence.next();
        sequence.next();
        sequence.next();

        //+ assert
        assert_eq!(sequence.peek(), Some(&4));
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