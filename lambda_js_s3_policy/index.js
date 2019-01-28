const crypto = require('crypto-js');
const moment = require('moment');

const bucket = 'powerofkek';
const API_KEY = process.env.API_KEY;
const API_SEC = process.env.API_SEC;
const region = 'eu-central-1';
const serviceName = 's3';
const d = moment().add(10, 'minutes');

exports.handler = async event => {
  var expiration = d.format('YYYY-MM-DDTHH:mm:ss\\Z'); // expire after 10 mins from now
  var isoDate = d.format('YYYYMMDDTHHmmss\\Z'); // ISO8601 Long format
  var simpleDate = d.format('YYYYMMDD');
  const credential = `${API_KEY}/${simpleDate}/${region}/${serviceName}/aws4_request`;

  const s3Policy = {
    expiration: `${expiration}`,
    conditions: [
      {
        bucket: `${bucket}`,
      },
      ['starts-with', '$key', `static/${event.params.querystring.filename}`],
      {
        acl: 'private',
      },
      ['starts-with', '$Content-Type', 'image/'],
      {
        'x-amz-server-side-encryption': 'AES256',
      },
      ['starts-with', '$x-amz-meta-tag', ''],
      {
        'x-amz-credential': credential,
      },
      {
        'x-amz-algorithm': 'AWS4-HMAC-SHA256',
      },
      {
        'x-amz-date': `${isoDate}`,
      },
    ],
  };

  const base64Policy = Buffer(JSON.stringify(s3Policy), 'utf-8').toString(
    'base64'
  );
  const signatureKey = getSignatureKey(
    API_SEC,
    simpleDate,
    region,
    serviceName
  );
  const signature = crypto
    .HmacSHA256(base64Policy, signatureKey)
    .toString(crypto.enc.Hex);

  const policy = {};
  policy['signature'] = signature;
  policy['key'] = `static/${event.params.querystring.filename}`;
  policy['acl'] = 'private';
  policy['base64Policy'] = base64Policy;
  policy['x-amz-credential'] = credential;
  policy['x-amz-algorithm'] = 'AWS4-HMAC-SHA256';
  policy['x-amz-date'] = `${isoDate}`;
  policy['x-amz-signature'] = signature;

  const response = {
    statusCode: 200,
    headers: { 'Access-Control-Allow-Origin': '*' },
    body: {
      policy,
    },
  };
  return response;
};

function getSignatureKey(key, dateStamp, regionName, serviceName) {
  const kDate = crypto.HmacSHA256(dateStamp, 'AWS4' + key);
  const kRegion = crypto.HmacSHA256(regionName, kDate);
  const kService = crypto.HmacSHA256(serviceName, kRegion);
  const kSigning = crypto.HmacSHA256('aws4_request', kService);

  return kSigning;
}
