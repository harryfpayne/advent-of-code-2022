use std::collections::{HashMap, HashSet};

fn main() {
    println!("{:?}", question_two(PUZZLE_INPUT))
}

fn get_letter_score(letter: char) -> i32 {
    let a = letter as i32;
    if a > 96 { // is lowercase
        return a - 96;
    }
    return a - 64 + 26 // is uppercase
}

fn question_one(input: &str) -> i32 {
    let each_line = input.split("\n").collect::<Vec<&str>>();

    let mut sum = 0;
    for rucksack in each_line {
        let (left, right) = rucksack.split_at(rucksack.len() / 2);

        let mut letters_in_left: HashSet<char> = HashSet::new();
        left.chars().for_each(|c| {
            letters_in_left.insert(c);
        });

        let mut duplicate_letter: Option<char> = None;
        right.chars().for_each(|c| {
            if letters_in_left.contains(&c) {
                duplicate_letter.insert(c);
            }
        });
        if duplicate_letter.is_some() {
            sum += get_letter_score(duplicate_letter.expect(""))
        }
    }
    sum
}

fn question_two(input: &str) -> i32 {
    let each_line = input.split("\n").collect::<Vec<&str>>();

    let mut sum = 0;
    for group in each_line.chunks_exact(3) {
        let mut items_seen: HashMap<char, [bool;3]> = HashMap::new();

        for (i, items) in group.into_iter().enumerate() {
            for item in items.chars() {
                let a = items_seen.entry(item).or_insert([false, false, false]);
                a[i] = true;
            }
        }

        let mut group_badge: Option<char> = None;
        for (item, freq) in items_seen {
            if freq[0] && freq[1] && freq[2] {
                group_badge.insert(item);
            }
        }

        if group_badge.is_some() {
            sum += get_letter_score(group_badge.expect(""))
        }
    }

    sum
}

const TEST_INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

const PUZZLE_INPUT: &str = "\
ZTmtZvZLTFNLMQMNRvZncdcHwcScJvcdHnVfwV
zqjqrzqjCqrjtqWhqChqrznhcfdfVnVSVgcffcgJcSgd
rtDGpzjjqjlrGsWqBWFRsbTPFQMTbRNRFmbs
FFlnlnVlmQqcBVhBRrSrGSwVdRJbztSt
NPPNsffWfNztRZSJNG
WpgpTTHDpgTDDpMLPGgMHslmBmmHcBQnFmcqhmnjlqQm
VlVNLlPQhtnDRPnP
QgqTffzZqgvgzWmqqZmGcDthtRFvnnFnhJtJJDDt
WGTBzSqBQTQmZBWHswpNbswLbSNCNl
PFzFQDdLjMzzQDhnDNmwZmqwRsmRRmMVNM
GtSbbtlttvvtBvtHBmqqNqVVwsVgCmRw
brlvctHfrlrqvGvcTpjDdFnLDhdfjLhQpn
fTqzgFrcmzTrTNgMzFzTrgtbMtVMVPSLVbvSStRttPVV
lsHnlQhplQpsHhlDJsswNwZPZZVNwvnSZSVvVS
BJpsGWhWQQHdGHlpWpfgrmqgNBmTTBqzTrzr
PLPPrDNHDBnrWNBmjDmjbqqgzjgjQC
GJpFwpvFFsJpwvwwGwJZJRptzdQSSZTmCjTQmTSQjjZSggQQ
ffwGGtJGCfLcNMNNfNLW
nlTzllGwlQHHGMSrrhQLcvbcghgcvL
fFNpttBRFFRNtJJnfNWmqbhhZrZrcjgpvLrvrvcvbr
JBftFFsVRWtNWRsmWtRfftGHGPDDCnTDHSCwzwVnwGSw
LsrZLbLmPSTZtPcc
zpnpljpdwhRpfNLlPWPtTPcnMWPcctTV
qlhNNjldqdpgqdfhjpphdsgmmrDBBJJQgHrmDHLQJb
ndDrsBbpqspHCjzVBCHjMj
LWFcQFQtVVdcCMHC
RLthwTWLTSLwTllRLLlLNlPpPGfrGnqDdGqsZqsZsZhb
jHnJLJrcBbrRHJpnLBBjdHbjfgvGwstwWwdsWWshfhFzFfhv
PZDQMNTMQwtgvGTGhf
lqlPPQZQVSQCPSCPDmLppjqRqHngLHjRgLnL
MfQrhdzNMfMPsNNmPmqPLCLCVLBnbP
WVtcVvJJVjvFZZmgGjnjLBjmnnHB
FFtcZRpWJcSDTwlVMhMDwrrDzM
bSMbrQGZwwqbrbGMdTQGMwQdfFBLDLHLHssDgsHHBJDsFfwL
lpPPCccvVPvccPzcWLDJFJBsBzJJHsqHjg
tnqlPlVlNvNZZSSrGGZb
NlBBBBBwmwcMgLwLVVLLLscHdnMDdbHJCbdvJbPdDDJHvn
ZRWQWSfGhtSjqZhffZhhRjtdvPFHvCJbFdCFFDdHJdPb
fpqGhQWWZRhfqRSqrpGllLcVLmscgrLwBmgCzN
wHHHwDNCzZwzZfpzwsswzfzvnvnMDMgMhMGGFGRVdMnvcF
mLtttqTPBtSJTSBQWlmcMvVdclnvhFggdGhRdn
QBLPSLQQWmLtTBtBSjZfsbwbHzpjGzHfpzzw
zDPBqBPqwzzsqlTzzPzqjttNbCPJVtPCbrvjvCZP
MFhMhGfFfGpWWpHhFfRpHjVNbDbJjVMjrVvVNtMtbt
gfRSmpggSFRgfGRphFGgfgBSBTqzSsdlTLqdsnsDBqLw
QTTdWZZZTrhCZrjhVt
SBJJNRRvvRpwwspNSpTPjCMVhhtCjGThjBjB
fJJpvRRwNRSwvzRzRvzSgSRNqFTFDbFmqncQTbdFfnbDQnLW
wSjwwjWhbhhjWjdqSVpqSndvMdmM
NTvPBrZgPPHDFrFHGvZHRBqVBpCQVRRdmpRppdqM
TgDHZPsNTWbbwsjtvw
HVRPRwJVppQNpGmvMvmqqq
WcttbWsWsdDdbFDSSWclZGvFLMvfvjfzNvLGzLmfqv
tglcmSldWlBQBThRQBhg
LDGGfPFLGsfFnGLfMzfJVccNzmcmwm
qggSRgtjgCRtdtNZCJMCcZwTZVcm
RBbHbbRRnLBDDrvN
pThhMtMtTsWspWTnGjpsVHPdjdjvHdgVvLDVVSjd
JQNrrwCFrSRLRVLnSN
JQcJFCfcnfQmrbblQqlthGqzpZpGTBMWTMqtst
DzVmzDDgsNdHHLGJhJppPPllqSgbql
jrBWjcQMCRchPShwSblpbW
ZjRrSCtnnrtBQTcrSZmGtGFFNDHmHVVmVtzz
GSbGbrTGRRScMlVFjfbqqjdF
tvDmZhtNtHDttBhCmHDJHNwlpjslMwfflvpqldsFpMlf
HdBhLDhJDLWWLDCSQGcrQPGccLTGcQ
tNzrCdJBrrtQdtgQdlQQtrnMZhMTqzHqqTFZZFqTHFMV
bwbsRDvfwfsfcfSFqMmqDqFVNMNVVH
PRvfbSjNvPWPlLllLJ
MjMRRNRMjZtGVGpBCMMCDV
vwfhFzhvcJQQwJlQSddzQwFWGqcDpCDqBqDTcGZTTVTCWG
dvFJzrfZJhddLNLPHsrtRbPr
BmbsFNBhszGgGtGl
MSHVwdSwZflGlcqqpM
QvwZCnnnSvZdCSWrNmDlPPBPDhPmjW
MDwpRzRwMzMsdVSjdMWWMQdW
gmlftnDtHnHHVWSWBjWjWgvW
HbtrhbbGRNDpbCDz
cjVGqQqVqBFhDtvB
ZnTTfTffZZmDhBtJZG
fNzbpzlHTlgHNzlRTlRNbHrWwrCwWSSdQddNjwVcjGSd
WWrPMhwhnjpSLCpDlSSW
sNGBGdmTbZNGsmbstNLZGZDBRSlqSRCfRSCqRRDDqCqp
NvvcGtsLdTNZmLTmdJPcnnMFjFMcrVJhMF
wMRQMSQZHznRsqRbWp
ddgDDhfDrrDdjthHmdHrzpbVCVWFpfVqCWpWqpFs
hdvNdHtjMLJZvJwM
nWzZtWzHzZWgQHMNLDMDfDBfQbdD
MCPmRGGhqdmNjbDN
FRPlMlRChCvFggWZsHsZHn
ZGhhjdgwgcZHsPnRnSnbWscn
tltlfMQQQftDFJpMQJsWJWJsnWBnRSBbrBWP
lfLDpQMFMNfLjGjPPgZzwh
HcmvWcqnHLLPDzPPHM
MGGSfdJRdCglfrLjrjrLzLPzlF
CsgRgfwgdCwhsssJBBwvcnQnQNMvQtQm
nrVbwgnSTSgSnrZpjpWWqmWNHlqqpV
BcBPhSdsDlHNtNlJNP
RLFCQLQRRsBDDcRdGDddhCRDfwzvSCnzrbfrMfMgznwwfbZv
HJLzLNwBNzNJLzBJztRGzQVnDgwsjbgwssZwnDZbDQ
PhvlSvvPfMRlDbDsggbSjQSg
mFfhMlMrfchvPPpFTPvMvPzLBHWRHJNtzJGzzHpzBtCC
FLsgSLzLswdFgLBbWZnJDWHcmZnnBjHM
rpbrrqfqpvCbqqQQvvpClblRDHmjmmjJJZZnWMJmmJmmcRMc
qTppQvCfhzSVVTzbdL
ThTJtlqfDrDtffwqRCFCCnLwdnmpzRdF
WPQSSFsGMgQZWvFQgZgZQcCdmmzLMLCpNzCCddmLzdzm
cvVPGPQQGsZsgGPjVFccbHhThJHhDqfhDDjHqDrr
CgnCCnPMnMtGHDbMFQ
zLpwpRTwwRwhRchHwmqmGvvGbqmTmNQqvq
sHdhzlwrRVrdLzRrprrfjgfjVBnBCfSBZZgPfJ
SPMdWwWPrZwdrrWrSPLFDfgbQDfwDFfFglDQ
qLGBtGLpjzqmvQbvvpDvfFpR
jhLqqHBLGjtLqSZCssSTZZMshP
MhJCpPDpRRFRzQQNNqbcZjNZmVhjNm
LLSlLnGDmNqLbNjb
sfBngrBSTTSnSGHlTsHBGsBpFFdRdQPDCFPWRMzzdQWM
ZQtmgtWfPcgPgcsb
pvMhFThpHVTvPbcFcFJLJDsd
MTVHchVVHjHHTcpjMVBjjnQnGqQZnlqBmrWQQBffQQ
HfcRNJpJfhCmpGSqqGNjsjBnQl
tTdPwwtLTrrTVPSnQsbGPsnnlFllFQ
VvvTSTwWMrSZVwwrrmRmczJchHcHCZhRzD
wfRwhmLRnvrHqHhV
bJlBHlWlHMBPJzDlzMMJJSBlFnrGVrqspGGvpVGPVpGsVqvp
DWdStDMztCCHgZCtmZ
MHdsznVDDfcjcjDcdDjmMSCQwQpCpFCvqSZQqFzQpQ
lhJnLJJnTNrWTRqvqqPrFwZvwqQp
NghhtJBtnWLRTNjmcMDtjdsMfGDH
jgsvPffVmHfDqPSrNwnQwnwNhSvw
PZbGbCdcGdRCGtntQLQQLLwtpLNw
RbcPFBRFcdZBBJDBmTHjsD
dTTFJdzhmmmQpzVz
jtNLcctGGjtfrnVMsNQNQVVWnv
GcrcrfLtDggLftDFhZFdJFHJBVFBgZ
TTbqTgqCqZCrwmhQnnmrgh
MhpfsMLhfmrznLrQrF
RsStRMtjpHMfDtWsWsNDppsqlZqBlhNlbcNdTPClPqcvBP
lRhZPgnpRGZlSrmsLSvSzLVl
wwHdHCfDQCJHdwdDMdHCcDsLmNVvzVsWrcVNcVzLbrLz
dMCCwCtJdwDQJMtjhnvhpPhRZBhR
pBqMZfDffmBnvnNmPt
rhwLHCChrLPCMNWMCNmW
GSMVRSVwHLMRJDQJTZlJZR
sfstzPGRRBSngMfQLNNqgWLQLZZNgq
scVDjjjCDTVhHlDhHdvvjwjHrZmWQmJmrJJWqNqLJbrcQqZq
VvHlVHldTjvhpVplhVThhwjlPFSzPfGzpGBnsRffRRBPPGGF
WNFNfnWTSLSJTnWShTvVZCnvrdPrZvddVCrt
QwsMjppcpHCPdHsvPZ
lcwMGgpcGbzQpMgQwbDjDQZSJTRfShffWNJSSNFFbhSb
JDNgTgqDTggQbQGbZDWbJmVJrPVfPjlPfPwlljJC
FZzHFSznZZtptHzcSmCVrwfPVcwwVrCcdm
nMSStvnZFSHpLLtBtMzHnMWQvNhgDgGNRGNhgqRWTgqg
SJcrhvbBLBLrDpllvnwHQRnllHnQ
ffsjfMMZfVdCCgCfgTzmzslRtwFwFtTnqqHTJRQnqRqq
CVPgmdggVjCJSrhrbrPrrSLW
LPtcLtgddLMRRCMRpTBRrZnppvvGRvBw
WNNJjDjqSjJSqWqzNqzlSlBTGGFvrppSrwTFpn
qbNDWNNHbJqVtctwVmsfLCLP
FvSSLMqgvVSQjQfgwpwWpj
BthszRPRRNbNtzmHRbHNRNPfwJGcsswWQpffJpfsJcQFwJ
bBtzPmRrbBRHtNCzPhqdCLFMLSSvdnvCTnML
VPHWJPDjVLDDjDSFDJhgdnNGdbblzTzNjlnNbl
ZprsRZMQwwmGZsvtQZgTfggqnbfdTzrbqlTd
GZMtsscmsRZswwBQHBhDDJJPPCPWSWCJ
mNDNNmmVMSVgGgGGqsqGLhQqsLGhLq
nZBTZpJPhCpnnrsqbbcfczJfFccz
HpBZZRPRHjnPPjrHnRtCZnBdShDVlMDNDgVmtmDdVDWSdN
tBftztmztGBBCBSGHBmhvHHcchbshhThpbLJHJ
wwzMrrMnQdldVdMvJTcLNnphphbLJv
ZzwPVrWQlwrdStGGCWqDSSGW
QwfrQPvhwPfzQrvWWpQpvVGGTDGsjbgNNcbfsGTsDFgG
CtddSdZMRRdnJhRnHtZtlRMbGGDjDgggjNTZDNgTGFgGjc
mdmdCnHhVWmLmwwL
zLcWSWFcPJLWrWLSZrJLjVjHtjVsrdtstHdtVQgg
nChlwwnmhlCNqhhjHMgDjVVdwMjdtd
CNnBmNNThhhdhCdlBGGlGvNpcJbJLSbcZzcFJzpJTWPbzc
LdPZTPVpLCVTtCNsNsfFnlDC
SMwqcqcWQMbMhWQzBnsNfsFwrnnNNlrs
WMMWhvQRNNNjvLgZ
DWFGzrtfsZHZZMLt
pNwNzNCNTpppmnvNMTLVjHLBLLjMRTLH
PPdlPmJJNNClDdcdDDfWhzrW
nSJVSHQQnwLThnhrML
ddsjfRdGZjmGjRTwwTZhwrMwWwtb
qCdfRdMmgssPfjsdjdPspBzQpScSSCBpzNBQzcQz
fJnmRMJrlrmRmTRmbqssWVdqNVQdswdNNb
GZggFHGhHHgHSFvtHPPPsfwgwNsVqjqNpNjNNssN
PPSPDDBPBmBMlLfmLr
BdqdCBqqCVPVTZBrlJcTcTJTcfcbwwmcgv
WjGGLzLMhpWQmRGhpHfbhcDhHHHhgcsbJD
tQzSGjWRzWBntntrZmVB
clfLQLgfzfTLDMwNrNrrNDGCGG
tmbpFtBvvmvdQQdFQwMJCG
SnbtnqnSbnQQsBqzgLgVsLZTLTPfVg
QnQBQQBVzqqzpmfgBpnqSDFPjhhWsFVhlsFFsDstFs
MGGrTHcvRTTrrrCDpjvWtFPlFlsvjp
bZbpTpJJBBQmBmJf
dNVgDdVtPcNPhgTLPLpTPlnTHHRn
WrvjvwjWwfwWjGJsrwBjQJjTQLbnSTTmpTRQSTClHTbLmn
JqWWGvBJBwGJfJJGvwqZZddFtDFhgDqZhHNM
VwJcNgbfvfJbfcmGLZfPhZLfZGTDhP
CnnrlBlprsBnzQFntnZLqDhZZqThWGtWWSPL
FllFdCjzlsCzjJNJGGJwHHVg
fTbVBmNJCJRVbTmbfJFHsDjQHDHQjnQRsvDn
cLWcrGtttddMPhrPhPtPrtzsnSQQBvHjFpFSpDHsMjnvjD
PrPgPdhGWLrrqgdqcVCffbNblBwfVmwqwC
gmBfbmlbBDqrdfrDcJ
PwVWrQphQWWhQsJFcMPqzDdcJq
QWCSSHpSQWCttQpCRCHNSlZBtrmBZTjvGgZjmBZjBn
JrnhMPvtVtPVHJGrBrQwTmQmRGGB
pSSZCFClCbbSLbljZlSlFFszzBwcZNwTzQNDmBwGTBNTBz
ldsCCjpFjCqdLgsFjpsLFQgtnfqtJvnMtnvhWnHHMnnVWV
hzNHzHjWNzwHjjhprpGvGgvGvvpv
PLBVVRPDLdrgCdMrdrdC
FmBTqTmLPrsFqTBDcTTVtNNJztqWQNQtWWtJqNNz
fFffFvFBgHQWHdvfGglBWbqbPSSbSwVntPhZwwbS
jJCMzNMCjNCLNMjjphPSPqhbqnwPZLSqZh
rNpJJDzpcNMzzdBnGcQTccBvgv
FRFMwsrzVtwstgbCHHJJPgNb
hfZGhZDnnTTHTCCNzJjH
hppDvznmZphZQVFQwFVWlRqFls
jrjrgdHdFBZsBlcCGghWNgpgbCCp
QwJJqQQMLwPTwLMMwzvzwwzhCWbvcNcCChWpchWbNGfFff
qqFQJTmwJSPjZsrlnBjdHm
QfffRppWfHpQSrWVpSGmGMMccSjBjmmGmc
qdzLvbwzwdsWwnFdBBcBhMjMDvBBcBhc
bPdZPqddqzFsZVRptZZQHVWNpN
BzBQQHNjTSzzJDDFZFgJDJ
qLvCnLpfCpqCnLJhntRglFncDrGrllmZFZlDrc
JpqvfhRhLddfpbbtsdJWjHSwHHTNSQNPTVHQTb
qVQCCVlQZWgHZMqgqWlrtScFwrmtmcJqSSsSJS
MzdnddpNLzhRpzbzNPPBbPScjcnmrwSFjSjSJFtrwjcF
ddTzRMPLdLbvhBRdLWGQClVVCWQZQDTGGf
DHHTsldDNdPnVDCRDCNHllHwcMpprSMpRmphhRWhrhmzSS
qJLBqQLvJLQgftgPjJrhrMMWSmWMmMrrSqrc
FjfFftgLBjJPBLQZGvvZtNClnTTNGCdHTbCCNsnslH
jHHNsNqhjsShsshdRRCDMfMbCWHBrGGC
TJQFmnpgmTpBDCgCMCDZCGDC
FwpQzwQTmVvwTJmFJzTcQSdhBNztNPNjSlqLhBNhSh
vBCfSDcRMfRcRHSRRZZtPwrWWNtdSmrNVGSdwm
gbLnTzqTbjhGqFzgWrtttQtrPQTtNPmP
zbhjzglgzzlBGcsflsCl
jNHDNNHjVGVDNQFDTQSFZzDQTd
vvLwhbnpvPPgClwnfFTmTZQgffFFtTfc
LrhrLvwrnJvhCHVVRZNMjsRJVB
ShfcBWfvdhhJBBVwCJjHTRNwRVNC
qQzlDqMDDDslPqGVLTNZVpPwTRZZpV
bgbDbsqzsDTcfrgFFdgg
vlRHvvHwvMMMTTlvjmRtBjSJmSnDnpdrpSSrJJnDQrLp
cPfCgZZzNzzcGhNszcTPNZLrnVSJpJhDrrhSSQSpDDQn
FbTgbGcgNgcFbPFHMqvjjRtjRWvFvt
fZTnqfFFDNglcjdjZcfLGQJBwrGGQwbGQTBBJz
VhvfvsPpWRChmphvRGBbBLGhSbLrBQBSwb
pCsCsHvsstPsfRMMMtmDqFjngdFZqDHFnNFFjl
PVVwffMlfGWMDDSwfDwVpRpsZRjBHgpSsjJSpBSp
TdnFbqTFdmbjctcqcbRBZtJJZgsBzBBzvgHJ
bNmbcqnnbNFLChCVCfjDfWlMjVDPCr
JBLLjBQccLLJhcBDDlSrdFDsVhrVsR
HgNWCgqWGbvCRRZvGWvZmszsSlrWdSdFrWzSldDF
qvGCZGHggRNHvGTgvLnBjpjjPJwTPjLcJj
GCGwQrwBZMZdGVdLzbqbbp
TRfTTCtgcDmhtDmsTDVSbvpLdNpzNVRqVdVL
CfDJjscgTcsjfhtFZljPZWZMWPlZjQ
WsrjjfRfjjZjwjWjBpDpVpVhMBsMMSBT
JgmqHnCHHPCCtCJgSZMgZppDTgzvzZMz
HCGCGqqqCtmnHnqLFHWjlFrWRRbfjZccNWrR
BJBfSfPLPvdhvrbbvpDsHgDTzgpdzgZpgN
cVcmRnCWCqGngHpZsZsTsqNN
jmGCVwWjjnWFMjGwcwmrLJbBJPbLSrPPTbFBTr
SPZmmtlmqjZlZMwhlrtggqGGcCLCpfGLgqdCqF
FBBVDVTVDJfgcddLCDdp
zzVHvVNTbWJJTTRbVWBFJbWHmwmSPlMjPSShjlhMhhrrml
GJZJZTsnhsDJtVZdtsZJZrBCQpLjQgBnrQgjCjQQQj
RPSfqcRShHbFcPSfBqLLprBCwrQQQCqg
zPzPRHbFPcRRRHPclMhSfvfZsJZVTTZsJNVMGWGVdGTWWD
lCZrCLWCwVllGzWPPBMTFpsbGdsTpsbNMgFb
RDjtjHcHjcHctDRtjnhtnHTgMqTMqhTbdbdZbgFqZdMN
vfDmvfjtmvtcHmjZfSRZHQzBLrVLCJLJLfJBPzVJlwPw
JMTHVZMWNSCwCwMS
nsddQbDCnQQdDBPdCQCSvwpDvwffhfSvpmppvp
BssqBFtqRHgTqVRC
cWTTthtrgrzpCdCddtpz
SSSLNJLGLSLfCJfJFQCJzQ
swMPMZVMMSlMSZMqVSSHznzcqgzWTHgTnhbnrr
RJjjgMjWShPqchtbVBPV
DDddwCnZMHLLvDnfLrvvbVbbBtpwVBVPwtVpbcbb
zrvnvLrlZCHrfZZLffHZHHTsTmsQgFQSFTMjjQlFTRmR
zhTTMLRVTzLbVqwVRJgDQQsSCgCDNgsZCpqp
rrmrBmmWrWnHjWnGWrnGnhDHSQgNSpQsCgSNgtNtDDHZ
fBrGPGmGPBcTMfLhJVTc
TbTCjTBSbCncHsDZDZPhZbzv
rMwplFdlWWJMJzhhpGtHtvHSSP
fMMfwWdWrNfJNdlVgMcTLTmLffjTqnLScCjL
SwhTllwJDwqqBWLBbNtfhjBB
mvllZMmRMZGFZRfctLWtWttzfNLR
MGvHMCGpVnFGlgvVFFnpnGmmsHrDJJdSsqPqJSqDJJdTTDqD
QTTcqJZJhHSpShhFpFzjDDwwsFzpdg
NBMnBvmBPvwrqvgvvqgD
bNNGmWmbbClQTQRqchhQbf";