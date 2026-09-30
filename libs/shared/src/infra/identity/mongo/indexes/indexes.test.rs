use super::*;

#[test]
fn issuer_subject_index_uses_the_issuer_field() {
    let index = IssuerSubjectIndexUnique::index();

    assert_eq!(index.keys, doc! { "issuer": 1, "subject": 1 });
}

#[test]
fn issuer_subject_principal_index_uses_the_issuer_field() {
    let index = IssuerSubjectPrincipalIdIndexUnique::index();

    assert_eq!(
        index.keys,
        doc! { "issuer": 1, "subject": 1, "principal_id": 1 }
    );
}
