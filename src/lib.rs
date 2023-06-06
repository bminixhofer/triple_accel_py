use pyo3::prelude::*;

#[pyfunction]
fn levenshtein(
    a: &[u8],
    b: &[u8]
) -> PyResult<u32> {
    Ok(
        triple_accel::levenshtein::levenshtein_exp(
            a,
            b,
        )
    )
}

#[pyfunction]
fn levenshtein_search(
    needle: &[u8],
    haystack: &[u8],
    mismatch_cost: Option<u8>,
    gap_cost: Option<u8>,
    start_gap_cost: Option<u8>,
    transpose_cost: Option<u8>,
) -> PyResult<(usize, usize, u32)> {
    Ok(
        if let Some(result) = triple_accel::levenshtein::levenshtein_search_simd_with_opts(
            needle,
            haystack,
            needle.len() as u32 / 2,
            triple_accel::SearchType::Best,
            triple_accel::levenshtein::EditCosts::new(
                mismatch_cost.unwrap_or(1),
                gap_cost.unwrap_or(1),
                start_gap_cost.unwrap_or(0),
                transpose_cost,
            ),
            false,
        )
        .next()
        {
            (result.start, result.end, result.k)
        } else {
            (0, 0, 0)
        },
    )
}

#[pymodule]
fn triple_accel_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(levenshtein_search, m)?)?;
    m.add_function(wrap_pyfunction!(levenshtein, m)?)?;
    Ok(())
}
