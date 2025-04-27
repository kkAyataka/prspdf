// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)


use super::super::base::*;
use super::super::utils::indent;
use super::super::utils::ToPdfString;

/// PDF32000-1:2008 7.10.2
///
/// ```text
/// <<
///   /FunctionType 0
///   /Domain [0.0 1.0 0.0 1.0 0.0 1.0 0.0 1.0 0.0 1.0 0.0 1.0 0.0 1.0]
///   /Range [0.0 1.0 0.0 1.0 0.0 1.0 0.0 1.0]
///   /Size [1 1 1 1 1 1 1]
///   /BitsPerSample 8
///   /Length 4
/// >>
/// stream
/// ... bytes stream
/// endstream
/// ```
pub struct Type0 {
    id: Id,
    domain: Vec<(f64, f64)>,
    range: Vec<(f64, f64)>,
    size: Vec<u32>,
    bits_per_sample: u8,
    samples: Vec<u8>,
}

impl Type0 {
    pub fn new_with_vec(
        in_domain: Vec::<(f64, f64)>,
        out_range: Vec::<(f64, f64)>,
        sample_sizes: Vec::<u32>,
        bit_per_sample: u8,
        samples: Vec<u8>,
    ) -> Type0 {
        Type0 {
            id: Id::new_0(),
            domain: in_domain,
            range: out_range,
            size: sample_sizes,
            bits_per_sample: bit_per_sample,
            samples,
        }
    }

    pub fn new<
        const IN_DOMAIN_SIZE: usize,
        const OUT_RANGE_SIZE: usize,
    >(
        in_domain: [(f64, f64); IN_DOMAIN_SIZE],
        out_range: [(f64, f64); OUT_RANGE_SIZE],
        sample_sizes: [u32; IN_DOMAIN_SIZE],
        bit_per_sample: u8,
        samples: Vec<u8>,
    ) -> Type0 {
        Type0 {
            id: Id::new_0(),
            domain: in_domain.to_vec(),
            range: out_range.to_vec(),
            size: sample_sizes.to_vec(),
            bits_per_sample: bit_per_sample,
            samples,
        }
    }

    fn get_stream_bytes(&self, indent_size: usize) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.append(&mut indent("stream\n", indent_size).into_bytes());
        bytes.append(&mut self.samples.clone());
        bytes.push("\n".as_bytes()[0]);
        bytes.append(&mut indent("endstream\n", indent_size).into_bytes());

        bytes
    }
}

impl PdfObject for Type0 {
    fn id(&self) -> &Id {
        &self.id
    }

    fn assign_ids(&mut self, id_factory: &mut IdFactory) {
        self.id = id_factory.next_id();
    }

    fn get_objects(&self) -> Vec<&dyn PdfObject> {
        vec![self]
    }

    fn to_bytes(&self, indent_depth: usize) -> Vec<u8> {
        let dict = indent(&format!(concat!(
            "<<\n",
            "  /FunctionType 0\n",
            "  /Domain {}\n",
            "  /Range {}\n",
            "  /Size {}\n",
            "  /BitsPerSample {}\n",
            "  /Length {}\n",
            ">>"),
            &self.domain.to_pdf_string(),
            &self.range.to_pdf_string(),
            self.size.to_pdf_string(),
            self.bits_per_sample,
            self.samples.len()
        ), indent_depth);

        let mut bytes: Vec<u8> = Vec::new();
        bytes.append(&mut format!("{} obj\n", self.id.to_string()).into_bytes());
        bytes.append(&mut dict.into_bytes());
        bytes.push("\n".as_bytes()[0]);
        bytes.append(&mut self.get_stream_bytes(indent_depth));
        bytes.append(&mut "endobj".to_string().into_bytes());

        bytes
    }
}

//------------------------------------------------------------------------------
// tests
//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_bytes() {
        let in_domain = [(0.0, 1.0), (0.0, 1.0), (0.0, 1.0), (0.0, 1.0), (0.0, 1.0), (0.0, 1.0), (0.0, 1.0)];
        let out_range = [(0.0, 1.0), (0.0, 1.0), (0.0, 1.0), (0.0, 1.0)];
        let sample_sizes = [1, 1, 1, 1, 1, 1, 1];
        let bit_per_sample = 8;
        let samples = vec![128u8, 128, 128, 128];
        let f = Type0::new(in_domain, out_range, sample_sizes, bit_per_sample, samples);

        let mut ok: Vec<u8> = Vec::new();
        ok.append(&mut concat!(
            "<<\n",
            "  /FunctionType 0\n",
            "  /Domain [0 1 0 1 0 1 0 1 0 1 0 1 0 1]\n",
            "  /Range [0 1 0 1 0 1 0 1]\n",
            "  /Size [1 1 1 1 1 1 1]\n",
            "  /BitsPerSample 8\n",
            "  /Length 4\n",
            ">>\n",
        ).to_string().into_bytes());
        ok.append(&mut "stream\n".to_string().into_bytes());
        ok.append(&mut vec![128u8, 128, 128, 128]);
        ok.append(&mut "endstream".to_string().into_bytes());

        assert_eq!(f.to_bytes(0), ok);
    }
}
