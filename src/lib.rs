const RECTANGULAR_LSAP_INFEASIBLE: i64 = -1;
const RECTANGULAR_LSAP_INVALID: i64 = -2;

///
pub fn solve(nr: usize, nc: usize, cost: Vec<f64>, maximize: bool, a: &mut Vec<usize>, b: &mut Vec<usize>) -> i64 {
    // handle trivial inputs
    if nr == 0 || nc == 0 {
        return 0;
    }

    // tall rectangular cost matrix must be transposed
    let transpose = nc < nr;

    // make a copy of the cost matrix if we need to modify it
    let temp: Vec<f64>;
    if transpose || maximize {
        todo!()        
    }

    // test for NaN and -inf entries
    for i in 0..(nr * nc) {
        if cost[i].is_nan() || cost[i].is_infinite() {
            return RECTANGULAR_LSAP_INVALID;
        }
    }

    // initialize variables
    let u = vec![0.0; nr];
    let v = vec![0.0; nc];
    let shortest_path_costs: Vec<f64> = Vec::with_capacity(nc);
    let path: Vec<i32> = vec![-1; nc];
    let col4row: Vec<i32> = vec![-1; nr];
    let row4col: Vec<i32> = vec![-1; nc];
    let SR: Vec<bool> = Vec::with_capacity(nr);
    let SC: Vec<bool> = Vec::with_capacity(nc);
    let remaining: Vec<usize> = Vec::with_capacity(nc);

    // iteratively build the solution
    for cur_row in 0..nr {
        let (sink, min_val) = augmenting_path(
            nc, cost, u, v, path, row4col, &mut shortest_path_costs, cur_row, SR, SC, remaining);

        if sink < 0 {
            return RECTANGULAR_LSAP_INFEASIBLE;
        }

        // update dual variables
        u[cur_row] += min_val;
        for i in 0..nr {
            if SR[i] && i != cur_row {
                u[i] += min_val - shortest_path_costs[col4row[i] as usize];
            }
        }

        for j in 0..nc {
            if SC[j] {
                v[j] -= min_val - shortest_path_costs[j];
            }
        }

        // augment previous solution
        let mut j = sink;
        loop {
            let i = path[j];
            row4col[j] = i;
            std::mem::swap(&mut col4row[i], j);
            if i == cur_row {
                break;
            }
        }
    }

    if transpose {
        let i = 0;
        todo!()
    } else {
        for i in 0..nr {
            a[i] = i;
            b[i] = col4row[i];
        }
    }

    return 0
}

fn augmenting_path(
    nc: usize, cost: &mut Vec<f64>, u: &mut Vec<f64>, v: &mut Vec<f64>, path: &Vec<i32>, 
    row4col: &Vec<i32>, shortest_path_costs: &mut Vec<f64>, i: usize, SR: &mut Vec<bool>, SC: &mut Vec<bool>, remaining: &Vec<usize>) {
    let min_val = 0.0;

    // Crouse's pseudocode uses set complements to keep track of remaining
    // nodes.  Here we use a vector, as it is more efficient in C++ (Rust?).
    let num_remaining = nc;
    for it in 0..nc {
        // Filling this up in reverse order ensures that the solution of a
        // constant cost matrix is the identity matrix (c.f. #11602).
        remaining[it] = (nc - it - 1);
    }

    SR.fill(false);
    SC.fill(false);
    shortest_path_costs.fill(f64::INFINITY);

    // find shortest augmenting path
    let sink = -1;
    while sink == -1 {
        let index = -1;
        let lowest = f64::INFINITY;
        SR[i] = true;

        for it in 0..num_remaining {
            let j = remaining[it];

            let r: f64 = min_val + cost[i * nc + j] - u[i] - v[j];
            if r < shortest_path_costs[j] {
                path[j] = i;
                shortest_path_costs[j] = r;
            }

            // When multiple nodes have the minimum cost, we select one which
            // gives us a new sink node. This is particularly important for
            // integer cost matrices with small co-efficients.
            if shortest_path_costs[j] < lowest ||
                (shortest_path_costs[j] == lowest && row4col[j] == -1) {
                lowest = shortest_path_costs[j];
                index = it;
            }
        }

        min_val = lowest;
        if min_val.is_infinite() { // infeasible cost matrix
            return (-1, min_val);  // returns min_val but it won't be used
        }

        let j = remaining[index];
        if row4col[j] == -1 {
            sink = j;
        } else {
            i = row4col[j];
        }

        SC[j] = true;
        remaining[index] = remaining[--num_remaining];
    }

    return (sink, min_val); // they assign p_minVal, we return instead
}