use approx::assert_relative_eq;
use lsap::{get_assigned_cost, solve, LSAPError};
use ndarray::{array, Array2};

fn solve_cost(matrix: &Array2<f64>, maximize: bool) -> Result<f64, LSAPError> {
    let shp = matrix.shape();
    Ok(get_assigned_cost(
        shp[0],
        shp[1],
        &matrix.iter().map(|x| *x).collect::<Vec<f64>>(),
        maximize,
    )?)
}

#[test]
fn test_solver() -> Result<(), LSAPError> {
    assert_relative_eq!(
        solve_cost(
            &array![
                [8.0, 5.0, 9.0, 9.0],
                [4.0, 2.0, 6.0, 4.0],
                [7.0, 3.0, 7.0, 8.0],
            ],
            false
        )?,
        15.0
    );

    assert_relative_eq!(
        solve_cost(
            &array![
                [8.0, 5.0, 9.0, 9.0],
                [4.0, 2.0, 6.0, 4.0],
                [7.0, 3.0, 7.0, 8.0],
            ],
            true
        )?,
        22.0
    );

    assert_relative_eq!(
        solve_cost(
            &array![
                [2.0, 5.0, 1.0, 1.0],
                [6.0, 8.0, 4.0, 6.0],
                [3.0, 7.0, 3.0, 2.0],
            ],
            false
        )?,
        8.0
    );

    Ok(())
}
