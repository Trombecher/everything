// SPEC - https://datatracker.ietf.org/doc/html/rfc5322#autoid-24

const optional = (s: string) => `(${s})?`;
const or = (...s: string[]) => `(${s.map(s => `(${s})`).join("|")})`;
const repeat = (s: string) => `(${s})*`;
const repeatN = (s: string, n: number) => `(${s}){${n}}`;
const repeatMin1 = (s: string) => `(${s})+`;
const repeatMax = (s: string, max: number) => `(${s}){0,${max}}`;
const repeatMinMax = (s: string, min: number, max: number) => `(${s}){${min},${max}}`;
const oxidize = (s: string) => s.replace("\\/", "/");

// ---

const ctext = "[!-'*-[\\]-~]";

// const CFWS = or(repeatMin1(optional(FWS) + comment) [FWS], FWS);

const DQUOTE = "[\"]";
const WSP = "[ \\t]";
const VCHAR = "[!-~]";

const FWS = optional(repeat(WSP) + "\\r\\n") + repeatMin1(WSP);

const quotedPair = "\\\\" + or(VCHAR, WSP);
// const ccontent = or(ctext, quotedPair, comment);
// const comment = "(" + repeat(optional(FWS) + ccontent) + optional(FWS) + ")";
const qtext = "[!#-[\\]-~]";
const qcontent = or(qtext, quotedPair);

const quotedString = DQUOTE + repeat(optional(FWS) + qcontent) + optional(FWS) + DQUOTE;

const atext = "[!#$%\w\d%'\\-+/=?^`{}|~]";
const atom = repeatMin1(atext);
const word = or(atom, quotedString);
const phrase = repeatMin1(word);
const dotAtomText = repeatMin1(atext) + repeat("." + repeatMin1(atext));
const dotAtom = dotAtomText;


const dtext = "[!-Z^-~]";
const domainLiteral = "[" + repeat(optional(FWS) + dtext) + optional(FWS) + "]";
const domain = or(dotAtom, domainLiteral);
const localPart = or(dotAtom, quotedString);
const addrSpec = localPart + "@" + domain;

const angleAddr = "<" + addrSpec + ">";
const displayName = phrase;

const nameAddr = optional(displayName) + angleAddr;
const mailbox = or(nameAddr, addrSpec);





const mailboxList = mailbox + repeat("," + mailbox);
const groupList = mailboxList;
const group = displayName + ":" + optional(groupList) + ";";
const address = or(mailbox, group);
const addressList = address + repeat("," + address);

console.log(address);