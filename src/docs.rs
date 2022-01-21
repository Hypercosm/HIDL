pub(crate) fn lower(docs: &str) -> String {
    let docs = docs.trim();
    let mut out = String::with_capacity(docs.len());

    for line in docs.lines() {
        let mut line = line.trim();
        if line.starts_with('*') {
            line = &line[1..];
        }
        // Only remove 1 space, because we need multipe for markdown
        if line.starts_with(' ') {
            line = &line[1..];
        }
        out.push_str(line);
        out.push('\n');
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn strip() {
        let doc = "\r\n         * Remove the object from the object list \r\n         *\r\n         * Future attempts to make calls on the object ID **MUST** fail.\r\n         *\r\n         * The resources associated with the object may be released, but the \r\n         * object ID may not be reused, we have plenty of them\r\n         *\r\n         * It is an error to remove the root singleton (id 0)\r\n         ";
        let lowered = "Remove the object from the object list\n\nFuture attempts to make calls on the object ID **MUST** fail.\n\nThe resources associated with the object may be released, but the\nobject ID may not be reused, we have plenty of them\n\nIt is an error to remove the root singleton (id 0)\n";
        assert_eq!(lower(doc), lowered);
    }
}
