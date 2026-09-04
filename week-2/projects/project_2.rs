fn main() {
     let toshiba_amt = 450_000;
     let mac_amt = 1_500_000;
     let hp_amt = 750_000;
     let acer_amt = 250_000;
     let dell_amt = 2_850_000;

     let toshiba_qty = 2;
     let mac_qty = 1;
     let hp_qty = 3;
     let acer_qty = 1;
     let dell_qty = 3;

     let sum = toshiba_amt + mac_amt + hp_amt + acer_amt + dell_amt;

     let qty = toshiba_qty + mac_qty + hp_qty + acer_qty + dell_qty;

     let average = sum / qty;
     println!("Average of the sales record is {}",average);
     println!("sum of the sales record is {}",sum);
}     


