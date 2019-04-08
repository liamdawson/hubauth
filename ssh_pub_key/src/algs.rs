// https://tools.ietf.org/html/rfc4253#section-6.6
// these don't all seem to be in standards, I may have missed some

const SSH_DSA: &str = "ssh-dss";
const SSH_RSA: &str = "ssh-rsa";
const SSH_ED25519: &str = "ssh-ed25519";
const SSH_ECDSA_SHA2_NISTP256: &str = "ecdsa-sha2-nistp256";
const SSH_ECDSA_SHA2_NISTP384: &str = "ecdsa-sha2-nistp384";
const SSH_ECDSA_SHA2_NISTP521: &str = "ecdsa-sha2-nistp521";
const PGP_SIGN_RSA: &str = "pgp-sign-rsa";
const PGP_SIGN_DSA: &str = "pgp-sign-dss";
