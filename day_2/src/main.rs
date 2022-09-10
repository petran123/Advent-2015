mod presents;

fn main() {
 

    // println!("{}",presents::INPUT);

    let presents = presents::get_presents();

    let (wrapping, ribbon) = presents.iter().fold((0, 0), |total, current| {
        let materials = current.get_material_dimensions();
        (materials.0 + total.0, materials.1 + total.1)
    });

    println!("total is {wrapping} square feet of wrapping, and {ribbon} feet of ribbon!");
}