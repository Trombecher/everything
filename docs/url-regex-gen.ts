// SPEC: https://datatracker.ietf.org/doc/html/rfc3986#autoid-70

const optional = (s: string) => `(${s})?`;
const or = (...s: string[]) => `(${s.map(s => `(${s})`).join("|")})`;
const repeat = (s: string) => `(${s})*`;
const repeatN = (s: string, n: number) => `(${s}){${n}}`;
const repeatMin1 = (s: string) => `(${s})+`;
const repeatMax = (s: string, max: number) => `(${s}){0,${max}}`;
const repeatMinMax = (s: string, min: number, max: number) => `(${s}){${min},${max}}`;
const oxidize = (s: string) => s.replace("\\/", "/");

// ---

const ALPHA = "[A-Za-z]";
const DIGIT = "\\d";
const unreserved = "[\\w\\.\\-~]";
const genDelims = "[:\\/\\?#[\\]@]";
const subDelims = "[!$&'()*+,;=]";
const reserved = or(genDelims, subDelims);
const HEXDIG = "[a-fA-F0-9]";

const decOctet = or(
    DIGIT,
    "[1-9]" + DIGIT,
    "1" + DIGIT + DIGIT,
    "2" + "[0-4]" + DIGIT,
    "25" + "[0-5]"
);

const pctEncoded = "%" + HEXDIG + HEXDIG;

const pchar = or(unreserved, pctEncoded, subDelims, ":", "@");
const segment = repeat(pchar);
const segmentNz = repeatMin1(pchar);
const segmentNzNc = repeatMin1(or(unreserved, pctEncoded, subDelims, "@"));

const pathAbempty = repeat("\\/" + segment);
const pathAbsolute = "\\/" + optional(segmentNz + repeat("\\/" + segment));
const pathNoscheme = segmentNzNc + repeat("\\/" + segment);
const pathRootless = segmentNz + repeat("\\/" + segment);
const pathEmpty = "";

const IPvFuture = `v${repeatMin1(HEXDIG)}\\.${repeatMin1(or(unreserved, subDelims, ":"))}`;

const h16 = repeatMinMax(HEXDIG, 1, 4);
const IPv4address = decOctet + "\\." + decOctet + "\\." + decOctet + "\\." + decOctet;
const ls32 = or(h16 + ":" + h16, IPv4address);

const IPv6address = or(
    repeatN(h16 + ":", 6) + ls32,
    "::" + repeatN(h16 + ":", 5) + ls32,
    optional(h16) + "::" + repeatN(h16 + ":", 4) + ls32,
    optional(repeatMax(h16 + ":", 1) + h16) + "::" + repeatN(h16 + ":", 3) + ls32,
    optional(repeatMax(h16 + ":", 2) + h16) + "::" + repeatN(h16 + ":", 2) + ls32,
    optional(repeatMax(h16 + ":", 3) + h16) + "::" + h16 + ":" + ls32,
    optional(repeatMax(h16 + ":", 4) + h16) + "::" + ls32,
    optional(repeatMax(h16 + ":", 5) + h16) + "::" + h16,
    optional(repeatMax(h16 + ":", 6) + h16) + "::"
);

const IPLiteral = `\\[${or(IPv6address, IPvFuture)}\\]`;

const regName = repeat(or(unreserved, pctEncoded, subDelims));
const host = or(IPLiteral, IPv4address, regName);
const port = repeat(DIGIT);
const userinfo = repeat(or(unreserved, pctEncoded, subDelims, ":"));
const authority = optional(userinfo + "@") + host + optional(":" + port);

const hierPart = or(
    `\\/\\/${authority}${pathAbempty}`,
    pathAbsolute,
    pathRootless,
    pathEmpty
);

const relativePart = or(
    `\\/\\/${authority}${pathAbempty}`,
    pathAbsolute,
    pathNoscheme,
    pathEmpty
);

const scheme = ALPHA + repeat(or(ALPHA, DIGIT, "\\+", "-", "\\."));

const path = or(
    pathAbempty,
    pathAbsolute,
    pathNoscheme,
    pathRootless,
    pathEmpty
);


const query = repeat(or(pchar, "\\/", "\\?"));
const fragment = repeat(or(pchar, "\\/", "\\?"));

const URI = `${scheme}:${hierPart}${optional(query)}${optional(`#${fragment}`)}`;

const absoluteURI = `${scheme}:${hierPart}${optional("\\?" + query)}`;

const relativeRef = `${relativePart}${optional(`\\?${query}`)}${optional(`#${fragment}`)}`;

const URIReference = or(URI, relativeRef);

console.log(oxidize(URI));