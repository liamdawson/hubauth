use data_encoding::BASE64;

pub struct AuthorizedPublicKeyV2 {
    pub options: Option<String>,
    pub alg: String,
    pub data: Vec<u8>,
    pub comment: Option<String>,
}

impl AuthorizedPublicKeyV2 {
    pub fn encoded_data(&self) -> String {
        BASE64.encode(self.data.as_slice())
    }
}

impl ToString for AuthorizedPublicKeyV2 {
    fn to_string(&self) -> String {
        let options = match &self.options {
            Some(v) => v.as_ref(),
            None => ""
        };
        let comment = match &self.comment {
            Some(v) => v.as_ref(),
            None => ""
        };

        format!("{} {} {} {}", options, self.alg, self.encoded_data(), comment).trim().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::AuthorizedPublicKeyV2;

    const SAMPLE_BYTES: &[u8] = &[0, 0, 0, 7, 115, 115, 104, 45, 114, 115, 97, 0, 0, 0, 3, 1, 0, 1, 0, 0, 2, 1, 0, 175, 123, 200, 28, 196, 68, 10, 106, 231, 188, 95, 41, 194, 15, 202, 198, 224, 170, 48, 137, 109, 209, 24, 82, 115, 57, 104, 64, 188, 40, 190, 89, 52, 101, 186, 53, 4, 243, 52, 75, 4, 27, 108, 93, 193, 194, 11, 206, 126, 55, 168, 62, 102, 51, 11, 38, 169, 117, 36, 136, 66, 157,
202, 47, 236, 186, 121, 140, 16, 34, 6, 47, 56, 237, 23, 160, 189, 176, 11, 122, 236, 123, 239, 131, 199, 26, 151, 58, 3, 236, 79, 50, 145, 3, 109, 243, 22, 141, 85, 240, 108, 241, 191, 7, 240, 62, 132, 13, 70, 255, 3, 4, 98, 103, 243, 145, 24, 184, 13, 76, 130, 75, 188, 15, 4, 213, 140, 247, 157, 116, 91, 19, 88, 103, 56, 228, 211, 48, 55, 163, 239, 222, 204, 43, 154, 121, 126, 49, 204, 17, 87, 145, 127, 56, 103, 114, 124, 108, 142, 253, 202, 228, 65, 134, 164, 166, 208, 197, 70, 202, 29, 145, 251, 14, 249, 97, 150, 218, 72, 104, 106, 129, 229, 151, 132, 171, 85, 209, 155, 85, 40, 227, 250, 254, 112, 218, 39, 137, 49, 217, 253, 145, 130, 234, 133, 252, 77, 178, 167, 249, 98, 202, 40, 32, 12, 108, 249, 222, 65, 159, 39, 153, 226, 206, 202, 250, 75, 0, 117, 168, 136, 124, 230, 111, 235, 18, 38, 38, 90, 38, 254, 102, 177, 65, 115, 82, 178, 121, 245, 125, 78, 156, 237, 115, 87, 107, 209, 23, 21, 70,
229, 192, 125, 191, 167, 196, 5, 110, 101, 176, 211, 47, 80, 54, 157, 143, 126, 131, 70, 242, 17, 85, 250, 142, 110, 174, 17, 162, 118, 201, 171, 28, 106, 172, 176, 208, 213, 227, 133, 59, 23, 71, 219, 248, 31, 157, 51, 227, 3, 41, 210, 93, 111, 180, 37, 199, 23, 119, 90, 192, 167, 76, 32, 120, 226, 107, 231, 222, 22, 106, 41, 79, 196, 25, 38, 110, 104, 152, 173, 241, 122, 100, 234, 193, 26, 221, 137, 106, 122, 33, 208, 3, 192, 221, 234, 214, 122, 160, 215, 2, 166, 228, 62, 244, 218, 28, 192, 114, 20, 73, 232, 19, 140, 32, 212, 65, 182, 67, 39, 251, 105, 144, 46, 54, 230, 42, 37, 232, 18, 110, 152, 198, 173, 81, 40, 33, 187, 195, 207, 112, 174, 111, 109, 25, 220, 107, 138, 66, 127, 119, 23, 29, 89, 11, 118, 129, 143, 88, 185, 186, 227, 177, 109, 193, 164, 186, 128, 249, 142, 247, 41, 125, 221, 191, 116, 147, 192, 168, 43, 218, 65, 44, 113, 147, 181, 59, 17, 54, 146, 161, 216, 96, 33, 129, 89, 100, 122, 6, 101, 203, 181, 13, 133, 169, 27, 94, 133, 52, 80, 15, 100, 17, 209, 162, 5, 214, 95, 251, 75, 78, 135, 169, 252, 2, 224, 161, 49, 87, 86, 198, 200, 136, 170, 244, 144, 159,
3, 58, 119, 41, 244, 191, 31, 133, 94, 69, 89, 103, 237, 104, 93, 185];
    const SAMPLE_BYTES_BASE64: &str = "AAAAB3NzaC1yc2EAAAADAQABAAACAQCve8gcxEQKaue8XynCD8rG4KowiW3RGFJzOWhAvCi+WTRlujUE8zRLBBtsXcHCC85+N6g+ZjMLJql1JIhCncov7Lp5jBAiBi847RegvbALeux774PHGpc6A+xPMpEDbfMWjVXwbPG/B/A+hA1G/wMEYmfzkRi4DUyCS7wPBNWM9510WxNYZzjk0zA3o+/ezCuaeX4xzBFXkX84Z3J8bI79yuRBhqSm0MVGyh2R+w75YZbaSGhqgeWXhKtV0ZtVKOP6/nDaJ4kx2f2RguqF/E2yp/liyiggDGz53kGfJ5nizsr6SwB1qIh85m/rEiYmWib+ZrFBc1KyefV9Tpztc1dr0RcVRuXAfb+nxAVuZbDTL1A2nY9+g0byEVX6jm6uEaJ2yascaqyw0NXjhTsXR9v4H50z4wMp0l1vtCXHF3dawKdMIHjia+feFmopT8QZJm5omK3xemTqwRrdiWp6IdADwN3q1nqg1wKm5D702hzAchRJ6BOMINRBtkMn+2mQLjbmKiXoEm6Yxq1RKCG7w89wrm9tGdxrikJ/dxcdWQt2gY9YubrjsW3BpLqA+Y73KX3dv3STwKgr2kEscZO1OxE2kqHYYCGBWWR6BmXLtQ2FqRtehTRQD2QR0aIF1l/7S06HqfwC4KExV1bGyIiq9JCfAzp3KfS/H4VeRVln7WhduQ==";

    #[test]
    pub fn it_encodes_an_authorized_key_correctly() {
        assert_eq!(AuthorizedPublicKeyV2{ options: None, alg: "ssh-rsa".to_owned(), data: SAMPLE_BYTES.to_vec(), comment: None }.to_string(), format!("ssh-rsa {}", SAMPLE_BYTES_BASE64));
    }
}
