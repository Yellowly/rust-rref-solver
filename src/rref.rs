pub fn convert(mut m: Vec<Vec<f64>>) -> Vec<Vec<f64>>{
    let mut lead: usize = 0;
    let row_count: usize = m.len();
    if row_count==0{return m}
    let column_count : usize = m[0].len();
    if column_count==0{return m}
    for r in 0..row_count{
        if column_count <= lead {
            return m //out of bounds
        }
        let mut i: usize = r;
        while m[i][lead] == 0.0{
            i+=1;
            if row_count == i{
                i=r;
                lead+=1;
                if column_count == lead{
                    return m//lead has gone through the whole matrix
                }
            }
        }
        if i != r {
            //swap rows i and r
            m.swap(i,r);
        }
        //divide row r by m[r, lead]
        let thing = m[r][lead];
        for n in 0..m[r].len(){
            m[r][n]/=thing;
        }
        for i in 0..row_count{
            if i != r {
                //Subtract M[i, lead] multiplied by row r from row i
                let temp = m[i][lead];
                for j in 0..column_count{
                    m[i][j]-=temp*m[r][j];
                }
            }
        }
    lead+=1_usize;
    }
    m
}