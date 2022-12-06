use std::collections::LinkedList;
use std::collections::HashSet;

static INPUT: &str = "jfnjjwbbqttplpvllqgllmdllfmllscssqmqzmmwzznqnwqnwnqnjjbdbpbtbdbzzzljljzjjpccrmmppzfpzfpfnfccfbbcqcrcffblfbftbfbtbwwwmgwmgmnngnllnfllhghcghhjppchcfcnfffllmmqbmmpwwwwlqwwqgqcqsqjqpqzqqdzdtztltslsljjfqfcqqgbqqqghqgqvgvrggqwggrgjgmmnrmmzgmzgzpzjjctcmtcmcnndppcvpvrrwvrvhrvhrhjhnjnvjnjrjggccvffnqqvfqvvnmvmqmfmfqqzfzbfzzzgpzpllrwwnpwpnwnwgwhhrrdnrrdjjzjszsjjbddcdbbvmbmqbqnbqbsqbsqqwbwhwggssdnnmttvnnvmnmhmfhhjchcttzdzdqqszzcwwhhwzhwhphqhcqqsggddfmmvzmzwmwfwzwrrbmrrnwnfnwnlwnwrwfwnnmtnnzwnwdnnbhhrphrhlhwllpmmbcbtbffmqffddjnjwwzpzfpptbbqqwbwzbzjbjmjljblbtlblqqhqbqggrngrgllbmbccmhmqmqwwqcqssqzzfjzjrjnnqrqssfnsnvvtgvvmsvsqqljjbsbrrjllvfvzfzmzhzzhthjhshlslfljfjqjpqpvvmpmhpmhmqqmmdwmddppjlplhlsstlssgnggrbblggffcdfdzzwqqtztqtwqtwtzzsjsbszsbsvbvwwjqjnnpdpccwssvdsdzzqbqbtbtqtmtltltvlvddzwzzfpzpjpgphprpgpqqwppdwpdplddvffcdffvpvqqgvqgvvrfvrvqrrcjcpjjpttftqqvjqvqsvqsvqssdpdbbbmcmscsddbhhgttwhhjlltqllnqntqtsscnntwwhswswlwggldltlttsjszsnznsznzccbtbblplnnmfmqmrrvjvhjhzhnzzgnnhrrdrllblpbllfdfjjssvnssvlsllnqqhwqhhhsgstsjstthrrhrghrhhfmhmwhwrwwsrwrfwwdnntqnnsvnvmnnfvnntztqzqhqnqjnnjfflbfllrsllqhqdqccgvgnvvcwcfccmssqnqhhqrrfrtrvvnjnpjnjjpplmlppvmpphjhppvhvdvssjcjrrtrdrrsvvbbjzzrtztgzghzhccwmccshhzbhhdwdwsdswwlcwllpblpphrppfhfnffrbbcgcmggnvnzzmvvcrrftrftrffcscvcsslbljlglzgzbzczszmsmbmnbbhdhvvsqvqhvvfrfddbpwgvztwwqcpzhhwnhphnrwldjmztsptbbgsqbqqccwbdqzvhfjlfldgphzbfprclgpfztbrgvsvfpghmdchscbdqjqgzvmrtdrfzbhgdvgznjcsmglcfwhdtpsljnvvzjcbbrczwtgpdmgpzhctvbbmvsjzthffsjqhfsdrclpqslbhnmpczwvggpzbjcchfjzjhhgtrmlgnzlndfvzrccgggrpmprbmjbfjjhzrhrtwgqdbgdlqghssrnmtmpvttcqwnwdzhgfnddgbqcsdvzvwqdnmmpwrwhfbqtcpqhvwbczrmjqzsntvdrncwjsmvvwcngrtlwtjmnctwrrtvphbjhlqmgzfsfsrblzzvmzlbhzjhwbdfpncdrfchmrqhspdszcjrnvwtmjzmsmzcdphsdzjgqswwrpdvlpvrdnhplnlmswvcrzlcmbtqtscjfwrnrctrvdqcqzwcvgvpdgrndrgsrvzftwpqjjgjhzwhvrjlqntdtcjdrqzhqlqqdffcgvttlhvwgggnwmdlvghfgjpsmntbvbjbbttrwsljwsrvtmznvqdptpwtdcwtcsfdjlmdqthqggjcptrqhbsbjzqqmvvjmgmppqmjmnjdqvspzlbgzjsjshpslmszqnzghsszpsmpzfcrqqjdwvtbnzstvvjzvtzgpptcmvmbvmpvpzvgfnwtlmdzhvhshtwvnbgwmtzqhcptflpqsqvmptchpfcbwhvjzdcnsnqrgdwfcthqfssnbqnvgvvhlzqfqmdlcwnshtvhhhpghjbmhdbfbqcvbnbvwbzcbbmjnrqmsdqnmnbsrvhggzsrlbwtfmgwrnlhrbrrrqdcspnrpnppngrtdqtbmbhcbjrlhpfjpdnfndmqvwvhlgmsntpwrlrwwqhwvzbpzqqggnbqlsjjqtbqjcdpmndgmtdhfbqrpdzzsnmhzmqqnbdqftqmnhfbdzdlfwgjsjhrcsmtfzgwbvbbzdrlbmcgmppqfppmbqrnsmrmhrdsvgcfmzpfnvrbbgfccfcbphszwdbnnwcjjvvlpdtfzgtslvgqwmsvlpzjcbqwqclrjrsgthhtqrqrhvsdfjntgllsvslrvdtnsdmrgtqcmswnqwlrwlfmcfftbjpvdnmczqzldsssszhjtqtqvqtwhjcqchjvqvntvzzzprbmjcctsqfdcvpbtsgnnsqtqnmjhrgqcjnzrdsgrbtdpqjbgcmnfwhnsrfwcdmncjzwcngfbmmrsbvgvvqpvrdjfsqwjdmqjdpzcbjjfmzjjgbnwqgrvpmbzdhsgtldrzvglscfwbmjltcrzrgdslgprwscwbrhtdtglznjdcvfjzjjqzntdqdbcrcbbmvnzdshjzcsfsgpghmgdqdwsnwjtvtbqbqccbcwjpnhdhzcvdssvnvqtvzwprhpgftdwwvgsbnlzzjppcrrwmrsthvjjrvrsdrbdqfgsjsmwfplpstrbnpdhhcblhjfwzngmhlwbvnfcbgwshspsbbgbldrvmcnczszpgnddrfwrtgcqjggrrcbjwrdjlrvtspbftrtjbzjwchpfnjctcjtwtpmtblczcftqlphdjczfrvtzlsglpvhqsqqblttdjrlczhrqsgpggmvnhpqtrfbpgvzftwtsmwhwswtpvtwnsshmlcffpcjshqhqqsjtpbgszscmcbnhjjtjmpgfdhgmljqmmwlfptstjjvqhcbjpjpwzwqflhslclzzjlmcttbsncqmfzhgnzwbdtnvfwbtztwbhtfsqjfzwmfflmbwnqzqhcjwdpbvngsgzlwvwcqhqjsndznbbdcqqhmjjpqjbsnvwztgmqwdcbbjvcndmhsbvbjnzlbscmgnjcrrwrfdljtcsgmwtffgcjflpzzdcnzvmrbnrjbbmhzqqjtgsrwqmmrhpndwlbnrtrhhpqlmdrcrtdmzsslrmffpftdjvfcpvvhzhjhqtrrsclvtbsccgmmqrjbqgbmpnbzlsncssdhmjppjptvddfgbbnjzjjldjlqjzhhttsclrmsgzctwjqqvtjlfzwgtffgrdjzwdcnrprlcswffghngrqcgsbzqhhvbfjtwcjlrrmbtqjdrgpnbftnmzqnndnqwgrqndlwmjnnspbhjlnzrnptnrmcjhpbfcqpvbchvdwthjlcrfpssgtfbsgfrftcrwttrspbsvzpvcczmdqslcdgfljvtjsdpjnwmdvfzfllrdrbgvpltzlqcrlwbncswhfvrdthspmhfhfdlvpbcqlmjfznhnqblffftgzqrtswnmtnvjprqqhhhvrscvbbzgmnlnprghfdjqbgjppjzjrnclfdssbmgspwcscnlcrrqmtlljrmcwgdgcqwvvjzvsjdjvsspszlcthwzrwqtzdgmqvnlvvzrvrpqqwswzcchncrpnjdmflvmhhwvrrstpvnszfrmvpdtpqpbdmwvvbbpjnwmtststtlcvqdnvqqphzlhhzbbbjssgdcnhlmwrzwvwmcmgrcngqzcnffqzfnvldpdjmsspgpbrzhnszfnljfcrgsjvqjjbstvghlcslhqlzhltpglwffrzfgjghssfgrptbnpbhqnhhfbjsnmsvltqpthdmzzrhrhhmzlplvrtdqfrfrppdpqnllblcfjqpdwznsbrhcncdpmztcrjrfnlwtznrmpbzqsbrqrbnthgfpshrdhnwjmrnsmsfqwdjsmsvhfrbdpjrwcvmdvvmdtfqjgmdsrqtctsdmznngbsrfjvhllgwt";
static PKG_WINDOW: usize = 4;
static MSG_WINDOW: usize = 14;

fn find_block_start(s: &str, window: usize) -> usize {
    let mut buf = LinkedList::new();

    for (i, c) in s.chars().enumerate() {
        if buf.len() == window {
            if check_unique(&buf, window) {
                return i;
            } else {
                buf.pop_front();
            }
        }
        buf.push_back(c);
    }

    0
}

fn check_unique(l: &LinkedList<char>, window: usize) -> bool {
    return HashSet::<char>::from_iter(l.iter().cloned()).len() == window;
}

fn main() {
    let test1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let test2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let test3 = "nppdvjthqldpwncqszvftbrmjlhg";
    let test4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let test5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvj";

    println!("test 1: {:?}, {:?}", find_block_start(test1, PKG_WINDOW), find_block_start(test1, MSG_WINDOW));
    println!("test 2: {:?}, {:?}", find_block_start(test2, PKG_WINDOW), find_block_start(test2, MSG_WINDOW));
    println!("test 3: {:?}, {:?}", find_block_start(test3, PKG_WINDOW), find_block_start(test3, MSG_WINDOW));
    println!("test 4: {:?}, {:?}", find_block_start(test4, PKG_WINDOW), find_block_start(test4, MSG_WINDOW));
    println!("test 5: {:?}, {:?}", find_block_start(test5, PKG_WINDOW), find_block_start(test5, MSG_WINDOW));


    println!("PUZZ INPUT: {:?}, {:?}", find_block_start(INPUT, PKG_WINDOW), find_block_start(INPUT, MSG_WINDOW));
}
