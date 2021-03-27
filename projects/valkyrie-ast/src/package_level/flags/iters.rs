use super::*;

impl<'a> IntoIterator for &'a FlagsDeclaration {
    type Item = EnumerateTerm;
    type IntoIter = EnumerateIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        EnumerateIterator { inner: self.body.terms.iter() }
    }
}

impl<'a> Iterator for EnumerateIterator<'a> {
    type Item = EnumerateTerm;

    fn next(&mut self) -> Option<Self::Item> {
        let term = self.inner.next()?;
        match &term {
            StatementNode::EnumerateField(field) => Some(EnumerateTerm::Field((**field).clone())),
            _ => self.next(),
        }
    }
}
