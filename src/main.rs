use blockchainlib::*;

fn main () {
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![
        Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 7,
                },
            ]
        }    
    ], difficulty );
    
    block.mine();
    println!("Mined genesis block {:?}", &block);
    
    let mut last_hash = block.hash.clone();

    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    println!("Verify: {}", &blockchain.verify());
    
    for i in 1..=10 {
        let mut block = Block::new(i, now(), last_hash, 0, "Another block".to_owned(), difficulty );
    
        block.mine();
        println!("Mined block {:?}", &block);

        last_hash = block.hash.clone();
        blockchain.blocks.push(block);

        println!("Verify: {}", &blockchain.verify());
    }

    blockchain.blocks[3].prev_block_hash[18] = 8;
    println!("Verify: {}", &blockchain.verify());
}
