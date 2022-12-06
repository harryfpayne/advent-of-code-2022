use std::collections::HashSet;

fn main() {
    println!("{}",question_two(PUZZLE_INPUT));
}

fn question_one(input: &str) -> i32 {
    let each_letter = input.split("").collect::<Vec<&str>>();
    let each_letter_with_index = each_letter.into_iter().enumerate().collect::<Vec<(usize, &str)>>();

    'window_loop: for window in each_letter_with_index.windows(4) {
        let mut set: HashSet<&str> = HashSet::new();
        let last_idx = window.last().unwrap();
        for (_, letter) in window {
            if letter.eq(&"") || set.contains(letter) {
                continue 'window_loop;
            }
            set.insert(letter);
        }
        return last_idx.0 as i32;
    }
    panic!("not found")
}

fn question_two(input: &str) -> i32 {
    let each_letter = input.split("").collect::<Vec<&str>>();
    let each_letter_with_index = each_letter.into_iter().enumerate().collect::<Vec<(usize, &str)>>();

    'window_loop: for window in each_letter_with_index.windows(14) {
        let mut set: HashSet<&str> = HashSet::new();
        let last_idx = window.last().unwrap();
        for (_, letter) in window {
            if letter.eq(&"") || set.contains(letter) {
                continue 'window_loop;
            }
            set.insert(letter);
        }
        return last_idx.0 as i32;
    }
    panic!("not found")
}

const TEST_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
const PUZZLE_INPUT: &str = "nfddjzjjjmrjjfttzctzzhqzqbbvhhcfcpcqpcqpccsmsvsswbwzwfffnvfvpfvffhnnrgngzgrrhvhfhvvmjvjcccvppqdppbnbjjzlzflfccjjtctqccrhrnhnqhqwwjssjjhpjjcqqdgghddhfdfbffdpfdfzdddthhrcrbrqrbqqbcbnnwbbzcbzccmqqwllrljjjpqpdpsddmbmccwwgngmmzzzpbpspnprnppprmmwfwffrrpsrrchrrrrdqdfddnvnjnppqmqhhpshhjmhjhzjhhhzllbpbnnngdgzgmmjvvprrhqrhqhpqhqqrnqnddvjvvftfggfcgfffgrghhbmbzzjczzcscrccgbbjbqjbjsbsqbqttbqbfqqqrzrmzrmmcwcmwmrwmwnmnfnjjbdjbbslltjjmgmrmllhbhsbsmsqmqllvjjrvjrjcrrtztjzjbjsbbrjjvbjjgqjjpjsszpszsnznllvmlmfmppbvbzvvddtbtrbttchthjthjhnjnqnqrrllnflnnljnnzjjswjwllpmmpnnqnrrndnnbwwmcmcmbbhjjbbfpfvpvttcvvhshqqznzqqdndldmllpgptgptpbblzlddgqqdmmvpvvrccfvvjrvjjgpggqcggjbgbqqcgqccgppffjppzczmzdmmqppwcwlcwwhccfpcphcctvtzthtzhhnmnvmnnvqnqnpnqqwqsqdsszwzbwwgcwcrrtprrfhhcmcqcdcvdcccnpptgpgwpprsprsrdrtdrtrntrtprtppstsccbwwvnvpnvpnptpqttzbzcznzhzhddfbflfcchvchhplhlwhhlhglhhwqqzlqzlzvlvtlvtvrttglttslslclslqssslclbbrjrpjjcscfsfppjlplhplptlltlflglffdttmffrggfjjmllbnlnbnznqnfffmhhnmmsgmsmfsszggpnnhmhnnpwnndqqlmqmnmtnmtnttvtztlzlnzzsdsnssvhsszcscffcpczpcpffpwfwnwhnntnssptthccbnndppgjgpjpssbnnpgnpppjtjqqzfqqrrbmmbdddzjzvzwzssnpsnspnpvnpppjppvsvffmpffbcffslfsfmsmmqzzmttnsnqsqcsqsbqbcclqqphpttvrtrlrhrthhlppscsrsjjvljlmlwlslblqqqgdsbrzwzjzwcjrwbpfmjtmdgjvbcfvtvmsfjtjcmtlzmsjlnmhcswcmjndggdsmqfmmdngjpvrsbhrchldnhdhfdlwccnfmgbwfzppgzzcvblvsmqbfghrgdwlzdcvpqthgbdlwbrfpsvlgpdqznftswgwvchjfrblbdsqjmzchfhlrjhpbrdgvgrrmhrnrdbrdsfsgzvqfdtnvddbtcjwphrhgpqlzjssrgzjcncjnbrzvhgbwpgtfnqhpspmgptzcgvjqgzpmwtjtzldqnclmplwdpzcppgcbrsnlzfgmlnljjhfzrftnhdfnqchgdqrfcjszvbmdrghwzmjnwgnrlptljzqrwsmcfwvbcjgsfdjhnqgzzztmcgmndbtdwvqmzlfcmhfgpqztwgjdccncdccpgbcvhfzbhhbjhgjpdzcmrwgtvrmzdwjtmlzllmgplpqjwwwvbrzgmvpcvwchcwfgbjtzrfctgvfrpphbnsbjlswrztqmchtzfstzdgdwwvhpdhztbmsrbqmndpgvnwwdtgzcddvmvbjstqmjvtzlzgrhzhvplwnpphctvtlvnpmwfzmqcvrnfmmgtsgbcjpffrvbpqpszfpjsjtzqmcnzhnjnpwtvgfqntnhhjhmbvmlvmqgggrnfmmmsvfsqffbvwtzlfhlbjqhrltzwfstvjqhbbblqdbcmgtjgmzdtpslbzsgnmpzsswjlwdpzpmmvmpntbhnqlwrcrfbghzhwlhhpjqztjjrrfscrtwtnlqlqmdbmbfnvngvvthhghgsvqlqvgvmtjmjtwpcznzqhhfpqqfphcdrtzjjhsffslthzwpmsnltnjmfgpsjgqzdwrtgnhflhrnjwqftpnqgptgvgjptzhhtqhtddsfhppmmqcrsnlnrswpjhqgzbpwzfzptzqzzwltlrmjwjrwdgvvzhshqqrhtzmvqpfljlvpmrzbqpscpvsfdbdbcbdwwhpmldlrgpwslzhtbpgtzscfhjlgwcgbhcbftpftvpggvcdvndqnfvfqbwrjtdcbwpsbqpzmwdhjpmjhjmlcdphrjbgsnmcmvfnrggfvttclmbvsfjpnbndbblnbdfqzmsldlswdrtzqsqppjshtlrtccthmmpjgddbbgfgthnzdffbtrpchzgbvqvjcsnpgbrzrczzmzrmhjrlvvgmsqddjsqmcqfmwnhznbczzjlpmhnfwjtrfgffsjdlwgdwwlvdpdlvszphntrvttczgnwffsdsvjmqbthgcgfjgznrfnbplbvgsjbsglhnrjpbldhmznqgqpvldvhcpmmwzfjdjdbnprtrrnwsszjhmngvmtsrqdqdsprwhjpsqwqbsdtpptwlbfbsvdgrplrvpnfbzwrdsdbvhpgwcnqvwdcswdmdltchnngpmlqvchbnrpzcnfhvlzbwbnmssbhpvvmpcwvrwzpfpssndwwfnrslpjwhwrfsswmgtszrhczcrclpldpwpghgptmzzjjjtvjcnncjpfbcvldbnlnqtsqdswcsrqcfgvwbwdvbdwwzndfvcstjbfngtqqwsbpdjdgqdlsnwgcvmmhrqcqvdbqdqczzwzlfgffbwzbfdnpvprzmqclllsdvctwjfgqbchhmsntlvnlspwtnhgshwrvzccfmfrscqwrvdccwqnrccctjrvvnqbrphrfvfrfldbbthhrdzvdmfbctsmvgwmvpdslgbcpqqdvpsjcdvmctwghdsjtmhhvdswbcvtmsnsztfghnnfhflmmnmdqpvpdplllzgqgnsjwsrgzfwhrwhcscvrgcrgjdghqjfbswtgjsvnpqznrvbdbrplwdmbqhtbcfccnpwqlsdstnpcfpbfgqrzmcqhflmcfvbbnwrrblnfslsrwpwlbvqfhgpdwzmgvftssrvdmhnmwdfqmsvqbltlmmwmjrrhgpgznqbwhcqgphvzqmntbbdhhpnlbbffjgmcdntgwmtblwlzrcdcdbtrllrdnznrrsglnwhtwbrfdrpvgqwsgzwghbtsfwqlchgsnvfmvnzntlsnlwrnjjltrpmhwnzmhrqdlvvzbfgwlwgdsgcjcjfvhbcjgzlqtsljvzcvlppqdszvdbsmgddrtmvbcpbpppcpvhzfsjrmtcpzljbhpnjjmcdwslrhslccpljrtvcscbcltpshpnrqvtdfzbbfqtpbvznvrbflwvbvrhqpzltsdrnqccsfgzzftvjfqslcnmfvwtpdbjhtzwrgvntgnfvqtdrjdgglvrnqfzsbhnvhcdbctthdrjnvwlcsjtmphpvlqjngwjnngmqqnslrrsdfpfbvsvcwjtfmwtbpnnghtvvwlphbnsgflvsfdcqrctvjfjrwqjdmbbcclwvlstbgbfqjgbpbqfwdbpmnvqnfpbhrfhwltmcszpwnvtvhrvpcqhzdppjwttlhgsnvmsrwrnwvgzpbwljjjsjzctftzftvmsstpjnzvmmrgbbpmfmfrszwjdgzfhpvsfdqbbhgvfvqrrtqwlwzwwsnnmmvmlwjzvgrwhmffzwrqwbcdtbtzpspbnqgprdqtzrpmgvnmbsnjnvtzgmhqqtrvltbsrwjlssncdppgpmzqzbzvbpjpfwmvgsbhffzpbctmqvfwhsgdjtwqhrhmgnqpvmpjzhppvcbrpwmdshzcrwzdzcjmhfvjgtbznsmdjphlssmlmbhtmnsqnjfsjwhjvgztnwhmnztqppchngdnhzwpsvqqpzdwgbhcbzvmbnqmghbhgvrqhtfzhgvqdbpvdrjsqrdnhqhrwdlczvtnzwfrqhnffwdvtrnqsmmcjtrhmgbwcmnzbbvdsrlbbtwslhghwprpglpq";
