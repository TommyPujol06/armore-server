use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use lib::auth::Claims;

pub static MOCK_PUBLIC_KEY: &str =
    "MIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEA6lORI0goLg5HUlkcnnAO
plNP9RF6QfHQ3EyS8aBEkxYVtQhvrG+cIN0X5ws48wqsCm3/fCQtwPghuDuCXRG8
rJTxWr5eUOy49HATRMHIdWSSG8sdz2SH//5lDu9u6u6QtUflYPEmNXCwZAzhhaWs
DhqYkBIbNKcCnspzI/itw7znaKdfSNQvXYWuT7LvDQAjorP+JJfy8JCQzHweT52F
BU/By9KOl6XyeOqwPc4gcKBj72KWSczwqhM0fxAFaKc/xSRxMYbKCPPGXq1TqS1l
xHLNHqMBvewxoM6eYHFvO5jekbLbdObh+irwwx1HlG24lYwGTc/7bDBkqMWTrvg+
VE4oCweIRi93pW21MLxUIZeH7G4gmPutwgY6gaZEYoKY9gvlupGU5TDZvF5Ny69F
rs3OJF4m9Lp7IQKdOCvnXnug6XB67vSc3a13kDygkTTfBVT8gdkb0yGkyhGwG2VA
9TGyxGgYFSVHHFW6vPl65b0ksLiED5twulJ4kzb4trEaayrqvYMgoNnq967RuOcp
nNQ885Uit5HTfNaU8/aRWnkDy/ItZCwzkABkP0GNLAKLKZ6hrtu5gHeVqi1xTvXx
pai+Emj+NmxkhpPsWFqCQznnLQ/BNBhQn/EtMU03W3Q6nA0QO1o37w8b/689dWwV
cMTE2BCIg/sAjsqQ8I9zEskCAwEAAQ==";

pub static MOCK_PRIVATE_KEY: &str = "-----BEGIN RSA PRIVATE KEY-----
MIIJKAIBAAKCAgEA6lORI0goLg5HUlkcnnAOplNP9RF6QfHQ3EyS8aBEkxYVtQhv
rG+cIN0X5ws48wqsCm3/fCQtwPghuDuCXRG8rJTxWr5eUOy49HATRMHIdWSSG8sd
z2SH//5lDu9u6u6QtUflYPEmNXCwZAzhhaWsDhqYkBIbNKcCnspzI/itw7znaKdf
SNQvXYWuT7LvDQAjorP+JJfy8JCQzHweT52FBU/By9KOl6XyeOqwPc4gcKBj72KW
SczwqhM0fxAFaKc/xSRxMYbKCPPGXq1TqS1lxHLNHqMBvewxoM6eYHFvO5jekbLb
dObh+irwwx1HlG24lYwGTc/7bDBkqMWTrvg+VE4oCweIRi93pW21MLxUIZeH7G4g
mPutwgY6gaZEYoKY9gvlupGU5TDZvF5Ny69Frs3OJF4m9Lp7IQKdOCvnXnug6XB6
7vSc3a13kDygkTTfBVT8gdkb0yGkyhGwG2VA9TGyxGgYFSVHHFW6vPl65b0ksLiE
D5twulJ4kzb4trEaayrqvYMgoNnq967RuOcpnNQ885Uit5HTfNaU8/aRWnkDy/It
ZCwzkABkP0GNLAKLKZ6hrtu5gHeVqi1xTvXxpai+Emj+NmxkhpPsWFqCQznnLQ/B
NBhQn/EtMU03W3Q6nA0QO1o37w8b/689dWwVcMTE2BCIg/sAjsqQ8I9zEskCAwEA
AQKCAgBvADkfkn3eG0tz2dyxvPljltGokKfudyNuSCPPrBDv8CVGRYHJGHHIK5O4
EdvfXa3TnvnIj8bQw3oNsLr3ZYCP7FpMlyNMiGaw/CpUhstzuLlxyw0LAl9eR98N
bSSIy4vnI/CntHRaGlCkhGmMisdvQvAER1912KtoFxTl9FY0A9dG/wonEMSDM+E3
xdZxvSAkYclBAm3FwWWmSCF/q2mo83glGlALzEOJPftQu8UoNQJCEtyIhzl2B3T1
v9wgECIoPDQWtvgbt4a/sLGR0XyEy7EZEzSvCCUWPOpPW0zK2YaNVEGbJgfkHtVA
SC1xRWyMAvG1iJFcVaxJOpbT6qpzExei2J/D7JKbwxOJN4/4uSRRmBS2lW4pg3kC
O0ZUCa/zrtAdvWjVLBptmf5WfVz3DaPty3SnBBuSfCpVWK2LVJIdHkcAk7xAccmB
yXlgJkqlgSDtKRzwnwwpL322yDnaSglOGXVNoJvRLonKtaTfAagkRAgtRZYhkYaR
V7Sqh2qrfKYl7GSBX15X4Q65U9u87ZuqI6pnc1WRWuxIuSStl3Ivr/hjF1nUp5+s
2D4v3LFk3JNiBowHsP26cQBAqwUx9JYSs/IMw7SjraqFfJakbDvpUQqO7mMUqSsR
SHKG240vSOWlsRPm9OBcfBxYXv54kQm/hFqupKa0A7IBrE8/5QKCAQEA+1HvdO8L
mT2r1ENXRidvnbPI9NKMHbBAq8T+Zerc9D/gFSDxIfa+A6qhKfIgucE5UfQYDgxZ
lS2meIkxh9JgvcwFN5pLUgR8NQPkuyFlzfG7njNVT5cM5q/vYM8LxpRzJuCk5ERn
MPJroZD8HnuIOYPokcZWbNXIoD7OWlA5WsW6GO+ghzyvO3MFhaIW9tN4tIV3TRd0
cBI0j07Hi2mT6GrZ4g2S8ouMVl5Utav8NSw0Z8J2/tieaJDH15D/FJYWWT8xYvRD
Mi6RybR2yU+YxkTf9iKxOdWgQjslRn0APNbRvC9BCKA433ihDo7pfib7n+yP5RW2
bVXh/vU0Rvwh1wKCAQEA7rCfGIB4NQRhgLZUst1CG8LQ0egZK0yrEseYxXY3LUIx
C8omh7Ms/69zbyty68qhN1GQOxKclzvtFGyos4tY6M58oar5jajppW0q02EO0ucn
j391sjqR5pNzEKzDFH8/ySsyQITYnLh8eFaAuvCuj4jA3EAf5bkS9rmPTz7t7cCR
YLvvZ0BCwvnxjjyqnf2OGY9tk279UpR5wlmcyvajrLGXj5Eq51YQiFXItIIEhxh5
U0T8jraYWL5zXrfzO7Ha+n6hFU3u7MjsVcxb6eAW31NCAFAdphGfHH9B85UD9rtZ
QuTdd8icWDsnC4tbRlB9pDYgSm+FYX1Q46uW9WkcXwKCAQBZjbvPJjMy2tf03j4m
IH1Ua7ELFE+bcKfKzXp9ZLBxVKWLwd5K5PqWoeGl6cKhjmnXeyxrLRlq4AZ24yRE
KsIQP7gINTHruu9rkMSbre3x8daSK+aVYtTVCxI4o+6lR1a1Hs2DDaDbvzZ9LwW3
8vr6y7c+4rb/NzqzZ03uvrGBV/3VTuYb6pLikzz/fl/Cel6DrR9y2A3EtagG/OJ8
GhX7dr/HHmEjjnhmelyjE/LeG69c3d27OANSbWzYsrFCa6zxBmSZx0J+ijum7Wh6
maNt2zMXKQuP+UCO+TZyJK7F/yJjdU8uPLGnZ/u0DVbEfi2hshEgZ5lG4piSWlvT
g5qnAoIBACSBIKPXqgq5s8vCluuQCTdDsToZHBhSLmu92PCCJugmEmgyL3hbf8tO
4wGijH3hTIywTbWrIAXFJXoVMCvdaOiaA9eZ1XbD2Y/yRTV0x5abwaIhpTdv27Z+
4H8xXNh6qZ+zmojhiFtXn7mryR5OBvRuvsgwinBQwMS5FmDRSAQvikxYEcIhwtQc
88OEJbfp+lyQYfrFY7rIeGKv39nupJOZyYsscXpV4EtpizuIEvcyWAPTLikJZf1U
i0J2MZ30kn/y8+HVPHA8PmDU003OdtEK47I2joJCeaobEFQXeza16m3foLtcFAUu
bsdGNdxoHP8LRB7+NVD2oHNhX8ICpFMCggEBAJdy1p+45jTbgsPc0neN1W+E0Io4
cIPP1wSs/g+0f9T+fNGNSdJxEvdomVcPCiNtlNQT2u9jd2mQ39HwZDRaCtPOGMpy
hd4WRGtt4EmXVFBc894iOAHKDvaPswkXjgjlfVpRIuaCHsBEwDwgUPam6z0Gft98
oNb/htQz3rKFN8DdJS9IMgQwc5TYMyICrPAjPA8yf8Ba/RlqXzfrNAOntAemEWSq
ZAkO3hOwA6+uvxhCjIxTa1BCBX06M6jk2e6sRa4OsXaRsQwsra6dQhAsixxkzfwt
J4Mqgb3VtQVgSJ+45V3diR9RLRfYYrX3LVJtRCC5U4yyeIoyQNMDhqY/VEM=
-----END RSA PRIVATE KEY-----\n";

pub static MOCK_PUBLIC_KEY_2: &str = "LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUlJQ0lUQU5CZ2txaGtpRzl\
3MEJBUUVGQUFPQ0FnNEFNSUlDQ1FLQ0FnQi9aMGkwU0VVaFl2Ly9hT1NJOFZ\
icQpOTDJOazNBZzBVNlJpVmxybHRaSlJCUCtId0NlK0lxR1J5SDNNZTFxaDB\
WQ0piTVZlMkFLT3B2bGZPQ2toajZxCmx2cUFzN01jNUMwUURhUEFNU3cycnl\
FeVVuNGFaOGVlMnlvK0s1VW9JUFdReS9RQVZzQ0g1MXhialcveVJIYmwKd0F\
XWE1meXRvZkF0WGtRTk9hQUJEOENLOTBqOXc2a0pCZWYvTGdXdFpqVDhQRW9\
XOHRTWXdsTjZLNUdpUnBLRwpXRnA2TEFPazBRYXJNK0huZG96clNBTnBQWTV\
BczFjaEF0V2tCSXc4eUJ2YmFVT0x1cjFIaERPNTF0L0pCNzRTCkJTeFlJWWh\
1NFRmVnpndmJVT2QxNjRMMTQwYXB3eHRjdml1a2NkSnlBOU1JVnN1VXZldkk\
5RGM3SzZXZlZnYmMKVS9ISkN6TDVHTmZGdWs4TjZYdmJseUc2YTV1di9IaUZ\
hT1lvalZ4ajAvWkdFUlBTeDdNQ2NOdHRiTVZjaE1oWQpManVMdUF5d0VaWGV\
ITHhEbEZMQUdIUG43aldYRm9HTVZuWkswSHJ0STJoZnVyQ0ppUXpaUjVyVTZ\
ibnZrMnphCmNLbm1QTXloS3lIU2ozVEdWSTNOY05DdTNqaTA1bnNBUTJxMGl\
ud1F6UHZXK1hlWi80dzViYTZzS0pleVk4UDIKOWFLVndMQXBOcHZqbkVvR1d\
pMTJDQUNzclVEOS9XSjloMGJ5SVIzNjg5Y0RKR1hHRXk2Y2hVQnQ4RFM0aG5\
GZgpTSkVNNlVHZTZtdlVMcW5Vdmkvdy9xMmlFOWVRTG5mSDFndU1JcjRZR3h\
1aTZLQjZvdnY1RHd3MUprZStlRmg4CmtZK3NLTXRmZHl2RElHaWFvb3VZMnd\
JREFRQUIKLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0tCg";

pub static MOCK_PRIVATE_KEY_2: &str = "-----BEGIN RSA PRIVATE KEY-----
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
-----END RSA PRIVATE KEY-----\n";

pub fn create_token(
    username: &str,
    device_id: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let exp: i64 = Utc::now().timestamp() + 10000;
    let claims = &Claims {
        username: username.to_string(),
        deviceId: device_id.to_string(),
        exp,
    };

    let header = Header {
        typ: Some("JWT".to_string()),
        alg: Algorithm::RS512,
        cty: None,
        jku: None,
        kid: None,
        x5u: None,
        x5t: None,
    };

    let encoding_key = EncodingKey::from_rsa_pem(&MOCK_PRIVATE_KEY.as_ref()).unwrap();

    encode(&header, &claims, &encoding_key)
}
