# What happens when you type a URL in browser?
Consider https://example.com/post. This tells the browser the protocol we're going to use is `HTTP` and we're going to use `TLS/SSL` to keep the communication encrypted, so security wouldn't be compromised. The `example.com` is the domain. And the post is the path to that domain.

In computer networks, computers will identify each other with IPs (whether IPv4 or IPv6). But this would be really hard for us humans to memorize these digits. So we invented domains. Therefore, there must be a mapping between these domains and IPs, so that when we request for a domain, the computer would know to send HTTP request to which IP. This mapping is called DNS. DNS is a network protocol for looking up for the IP of a given domain.

So the first thing browser will do is to lookup for the IP of the given domain. There are several layers that DNS is cached (to improve performance). The browser cache, OS cache, internal network cache, ISP cache. The DNS request would go through layers one by one untill it finds the corresponding IP. If its request didn't resolve in any of these cached layers, it would go through a process called DNS recursive lookup until it gets resolved.

When the cache comes up empty, your ISP's resolver (the "Recursive Resolver") becomes a detective. It doesn't know the IP, but it knows who might know.

It follows a hierarchy, reading the domain from right to left (.com → example.com):

1. Ask the Root Server (.): "I don't know," says the Root, "but here is the address for the .com manager (TLD Server)."
2. Ask the TLD Server (.com): "I don't have the IP," says the TLD Server, "but I know example.com uses AWS Route53. Here is the address for their Authoritative Name Server."
3. Ask the Authoritative Name Server: This server actually holds the record. It says, "Yes, example.com is at 93.184.216.34. Here you go."
4. Result: Your ISP saves this (caches it) and sends the IP back to your browser.

When it finds to what IP address it should send request to, it will start a TCP connection. First of all, three-way handshake would happen to establish the connection (SYN, SYN-ACK, ACK). Then SSL negotation would take place to exchange keys that are necessary for having an encrypted connection.

Then finally the HTTP request would be sent. The server then would respond. Supposing the request was successful, the status code of our http request would be 200. Its response would have a body which is the real response we're looking for. Suppose it's an html.

Browser would render this html and show it to the user. It also check for any other resource that it would need to load this webpage completely. For example js, css, font, image files. It then send http requests for those as well to fetch their content as well.

