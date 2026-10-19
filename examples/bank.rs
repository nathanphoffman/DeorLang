// bank.rs
// Coding test: run a list of text commands against a set of bank accounts.
// A command is rejected if it names an unknown account, would overdraw an
// account, or isn't a known command. Prints a log line per command, then the
// final balances.

struct Account {
    owner: String,
    balance: i64,
}

fn find_account(accounts: &[Account], wanted: &str) -> Option<usize> {
    accounts.iter().position(|account| account.owner == wanted)
}

fn apply(accounts: &mut Vec<Account>, command: &str) -> Result<(), &'static str> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() < 3 {
        return Err("bad command");
    }

    let action = parts[0];
    let owner = parts[1];
    let amount: i64 = parts[parts.len() - 1].parse().unwrap_or(0);
    let source = find_account(accounts, owner);

    if action == "open" {
        if source.is_some() {
            return Err("account exists");
        }
        accounts.push(Account {
            owner: owner.to_string(),
            balance: amount,
        });
        return Ok(());
    }

    let source = source.ok_or("unknown account")?;

    if action == "deposit" {
        accounts[source].balance += amount;
        return Ok(());
    }

    // withdraw and transfer both take money out of the source account
    if accounts[source].balance < amount {
        return Err("insufficient funds");
    }

    match action {
        "withdraw" => accounts[source].balance -= amount,
        "transfer" => {
            let target = find_account(accounts, parts[2]).ok_or("unknown account")?;
            accounts[source].balance -= amount;
            accounts[target].balance += amount;
        }
        _ => return Err("unknown command"),
    }

    Ok(())
}

fn main() {
    let commands = [
        "open alice 100",
        "open bob 50",
        "open cara 0",
        "deposit alice 25",
        "withdraw bob 80",
        "transfer alice cara 60",
        "transfer cara dave 10",
        "refund bob 5",
        "withdraw alice 65",
    ];

    let mut ledger: Vec<Account> = Vec::new();
    let mut rejected = 0;
    for command in commands {
        match apply(&mut ledger, command) {
            Ok(()) => println!("{} -> ok", command),
            Err(reason) => {
                rejected += 1;
                println!("{} -> rejected: {}", command, reason);
            }
        }
    }

    println!();
    println!("Final balances:");
    for account in &ledger {
        println!("{:<8}{}", account.owner, account.balance);
    }

    println!("Rejected commands: {}", rejected);
}
