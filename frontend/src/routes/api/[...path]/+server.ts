import type { RequestHandler } from './$types';

const BACKEND_URL = process.env.BACKEND_URL || 'http://localhost:3000';

async function proxy(event: Parameters<RequestHandler>[0]) {
  const targetUrl = new URL(event.url.pathname + event.url.search, BACKEND_URL).toString();
  const requestHeaders = new Headers(event.request.headers);

  requestHeaders.delete('host');
  requestHeaders.delete('content-length');
  requestHeaders.delete('connection');
  requestHeaders.delete('keep-alive');
  requestHeaders.delete('proxy-authenticate');
  requestHeaders.delete('proxy-authorization');
  requestHeaders.delete('te');
  requestHeaders.delete('trailers');
  requestHeaders.delete('transfer-encoding');
  requestHeaders.delete('upgrade');

  const init: RequestInit = {
    method: event.request.method,
    headers: requestHeaders,
    redirect: 'manual'
  };

  if (event.request.method !== 'GET' && event.request.method !== 'HEAD') {
    init.body = await event.request.arrayBuffer();
  }

  const response = await fetch(targetUrl, init);
  const responseHeaders = new Headers(response.headers);

  responseHeaders.delete('content-encoding');
  responseHeaders.delete('content-length');
  responseHeaders.delete('transfer-encoding');

  return new Response(response.body, {
    status: response.status,
    statusText: response.statusText,
    headers: responseHeaders
  });
}

export const GET = proxy;
export const POST = proxy;
export const PUT = proxy;
export const PATCH = proxy;
export const DELETE = proxy;
export const OPTIONS = proxy;
export const HEAD = proxy;