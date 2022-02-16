(function() {var implementors = {};
implementors["wireguard_builder_rs"] = [{"text":"impl&lt;DB:&nbsp;Backend&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/allowedips/columns/struct.star.html\" title=\"struct wireguard_builder_rs::schema::allowedips::columns::star\">star</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/allowedips/struct.table.html\" title=\"struct wireguard_builder_rs::schema::allowedips::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::allowedips::columns::star"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/allowedips/columns/struct.id.html\" title=\"struct wireguard_builder_rs::schema::allowedips::columns::id\">id</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/allowedips/struct.table.html\" title=\"struct wireguard_builder_rs::schema::allowedips::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::allowedips::columns::id"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/allowedips/columns/struct.ip.html\" title=\"struct wireguard_builder_rs::schema::allowedips::columns::ip\">ip</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/allowedips/struct.table.html\" title=\"struct wireguard_builder_rs::schema::allowedips::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::allowedips::columns::ip"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/allowedips/columns/struct.subnetmask.html\" title=\"struct wireguard_builder_rs::schema::allowedips::columns::subnetmask\">subnetmask</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/allowedips/struct.table.html\" title=\"struct wireguard_builder_rs::schema::allowedips::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::allowedips::columns::subnetmask"]},{"text":"impl&lt;DB:&nbsp;Backend&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/allowedipsclients/columns/struct.star.html\" title=\"struct wireguard_builder_rs::schema::allowedipsclients::columns::star\">star</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/allowedipsclients/struct.table.html\" title=\"struct wireguard_builder_rs::schema::allowedipsclients::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::allowedipsclients::columns::star"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/allowedipsclients/columns/struct.id.html\" title=\"struct wireguard_builder_rs::schema::allowedipsclients::columns::id\">id</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/allowedipsclients/struct.table.html\" title=\"struct wireguard_builder_rs::schema::allowedipsclients::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::allowedipsclients::columns::id"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/allowedipsclients/columns/struct.ip_id.html\" title=\"struct wireguard_builder_rs::schema::allowedipsclients::columns::ip_id\">ip_id</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/allowedipsclients/struct.table.html\" title=\"struct wireguard_builder_rs::schema::allowedipsclients::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::allowedipsclients::columns::ip_id"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/allowedipsclients/columns/struct.client_id.html\" title=\"struct wireguard_builder_rs::schema::allowedipsclients::columns::client_id\">client_id</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/allowedipsclients/struct.table.html\" title=\"struct wireguard_builder_rs::schema::allowedipsclients::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::allowedipsclients::columns::client_id"]},{"text":"impl&lt;DB:&nbsp;Backend&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/clients/columns/struct.star.html\" title=\"struct wireguard_builder_rs::schema::clients::columns::star\">star</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/clients/struct.table.html\" title=\"struct wireguard_builder_rs::schema::clients::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::clients::columns::star"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/clients/columns/struct.id.html\" title=\"struct wireguard_builder_rs::schema::clients::columns::id\">id</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/clients/struct.table.html\" title=\"struct wireguard_builder_rs::schema::clients::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::clients::columns::id"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/clients/columns/struct.name.html\" title=\"struct wireguard_builder_rs::schema::clients::columns::name\">name</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/clients/struct.table.html\" title=\"struct wireguard_builder_rs::schema::clients::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::clients::columns::name"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/clients/columns/struct.description.html\" title=\"struct wireguard_builder_rs::schema::clients::columns::description\">description</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/clients/struct.table.html\" title=\"struct wireguard_builder_rs::schema::clients::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::clients::columns::description"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/clients/columns/struct.keepalive_interval.html\" title=\"struct wireguard_builder_rs::schema::clients::columns::keepalive_interval\">keepalive_interval</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/clients/struct.table.html\" title=\"struct wireguard_builder_rs::schema::clients::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::clients::columns::keepalive_interval"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/clients/columns/struct.dns_server_id.html\" title=\"struct wireguard_builder_rs::schema::clients::columns::dns_server_id\">dns_server_id</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/clients/struct.table.html\" title=\"struct wireguard_builder_rs::schema::clients::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::clients::columns::dns_server_id"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/clients/columns/struct.keypair_id.html\" title=\"struct wireguard_builder_rs::schema::clients::columns::keypair_id\">keypair_id</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/clients/struct.table.html\" title=\"struct wireguard_builder_rs::schema::clients::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::clients::columns::keypair_id"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/clients/columns/struct.vpn_ip_address_id.html\" title=\"struct wireguard_builder_rs::schema::clients::columns::vpn_ip_address_id\">vpn_ip_address_id</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/clients/struct.table.html\" title=\"struct wireguard_builder_rs::schema::clients::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::clients::columns::vpn_ip_address_id"]},{"text":"impl&lt;DB:&nbsp;Backend&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/dns_servers/columns/struct.star.html\" title=\"struct wireguard_builder_rs::schema::dns_servers::columns::star\">star</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/dns_servers/struct.table.html\" title=\"struct wireguard_builder_rs::schema::dns_servers::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::dns_servers::columns::star"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/dns_servers/columns/struct.id.html\" title=\"struct wireguard_builder_rs::schema::dns_servers::columns::id\">id</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/dns_servers/struct.table.html\" title=\"struct wireguard_builder_rs::schema::dns_servers::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::dns_servers::columns::id"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/dns_servers/columns/struct.name.html\" title=\"struct wireguard_builder_rs::schema::dns_servers::columns::name\">name</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/dns_servers/struct.table.html\" title=\"struct wireguard_builder_rs::schema::dns_servers::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::dns_servers::columns::name"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/dns_servers/columns/struct.description.html\" title=\"struct wireguard_builder_rs::schema::dns_servers::columns::description\">description</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/dns_servers/struct.table.html\" title=\"struct wireguard_builder_rs::schema::dns_servers::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::dns_servers::columns::description"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/dns_servers/columns/struct.ip_address.html\" title=\"struct wireguard_builder_rs::schema::dns_servers::columns::ip_address\">ip_address</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/dns_servers/struct.table.html\" title=\"struct wireguard_builder_rs::schema::dns_servers::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::dns_servers::columns::ip_address"]},{"text":"impl&lt;DB:&nbsp;Backend&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/keypairs/columns/struct.star.html\" title=\"struct wireguard_builder_rs::schema::keypairs::columns::star\">star</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/keypairs/struct.table.html\" title=\"struct wireguard_builder_rs::schema::keypairs::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::keypairs::columns::star"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/keypairs/columns/struct.id.html\" title=\"struct wireguard_builder_rs::schema::keypairs::columns::id\">id</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/keypairs/struct.table.html\" title=\"struct wireguard_builder_rs::schema::keypairs::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::keypairs::columns::id"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/keypairs/columns/struct.public_key.html\" title=\"struct wireguard_builder_rs::schema::keypairs::columns::public_key\">public_key</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/keypairs/struct.table.html\" title=\"struct wireguard_builder_rs::schema::keypairs::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::keypairs::columns::public_key"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/keypairs/columns/struct.private_key.html\" title=\"struct wireguard_builder_rs::schema::keypairs::columns::private_key\">private_key</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/keypairs/struct.table.html\" title=\"struct wireguard_builder_rs::schema::keypairs::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::keypairs::columns::private_key"]},{"text":"impl&lt;DB:&nbsp;Backend&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/servers/columns/struct.star.html\" title=\"struct wireguard_builder_rs::schema::servers::columns::star\">star</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/servers/struct.table.html\" title=\"struct wireguard_builder_rs::schema::servers::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::servers::columns::star"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/servers/columns/struct.id.html\" title=\"struct wireguard_builder_rs::schema::servers::columns::id\">id</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/servers/struct.table.html\" title=\"struct wireguard_builder_rs::schema::servers::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::servers::columns::id"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/servers/columns/struct.name.html\" title=\"struct wireguard_builder_rs::schema::servers::columns::name\">name</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/servers/struct.table.html\" title=\"struct wireguard_builder_rs::schema::servers::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::servers::columns::name"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/servers/columns/struct.description.html\" title=\"struct wireguard_builder_rs::schema::servers::columns::description\">description</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/servers/struct.table.html\" title=\"struct wireguard_builder_rs::schema::servers::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::servers::columns::description"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/servers/columns/struct.forward_interface.html\" title=\"struct wireguard_builder_rs::schema::servers::columns::forward_interface\">forward_interface</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/servers/struct.table.html\" title=\"struct wireguard_builder_rs::schema::servers::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::servers::columns::forward_interface"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/servers/columns/struct.external_ip_address.html\" title=\"struct wireguard_builder_rs::schema::servers::columns::external_ip_address\">external_ip_address</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/servers/struct.table.html\" title=\"struct wireguard_builder_rs::schema::servers::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::servers::columns::external_ip_address"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/servers/columns/struct.keypair_id.html\" title=\"struct wireguard_builder_rs::schema::servers::columns::keypair_id\">keypair_id</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/servers/struct.table.html\" title=\"struct wireguard_builder_rs::schema::servers::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::servers::columns::keypair_id"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/servers/columns/struct.vpn_ip_address_id.html\" title=\"struct wireguard_builder_rs::schema::servers::columns::vpn_ip_address_id\">vpn_ip_address_id</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/servers/struct.table.html\" title=\"struct wireguard_builder_rs::schema::servers::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::servers::columns::vpn_ip_address_id"]},{"text":"impl&lt;DB:&nbsp;Backend&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/users/columns/struct.star.html\" title=\"struct wireguard_builder_rs::schema::users::columns::star\">star</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/users/struct.table.html\" title=\"struct wireguard_builder_rs::schema::users::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::users::columns::star"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/users/columns/struct.id.html\" title=\"struct wireguard_builder_rs::schema::users::columns::id\">id</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/users/struct.table.html\" title=\"struct wireguard_builder_rs::schema::users::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::users::columns::id"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/users/columns/struct.username.html\" title=\"struct wireguard_builder_rs::schema::users::columns::username\">username</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/users/struct.table.html\" title=\"struct wireguard_builder_rs::schema::users::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::users::columns::username"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/users/columns/struct.password.html\" title=\"struct wireguard_builder_rs::schema::users::columns::password\">password</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/users/struct.table.html\" title=\"struct wireguard_builder_rs::schema::users::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::users::columns::password"]},{"text":"impl&lt;DB:&nbsp;Backend&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_ip_addresses/columns/struct.star.html\" title=\"struct wireguard_builder_rs::schema::vpn_ip_addresses::columns::star\">star</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_ip_addresses/struct.table.html\" title=\"struct wireguard_builder_rs::schema::vpn_ip_addresses::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::vpn_ip_addresses::columns::star"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_ip_addresses/columns/struct.id.html\" title=\"struct wireguard_builder_rs::schema::vpn_ip_addresses::columns::id\">id</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_ip_addresses/struct.table.html\" title=\"struct wireguard_builder_rs::schema::vpn_ip_addresses::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::vpn_ip_addresses::columns::id"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_ip_addresses/columns/struct.vpn_network_id.html\" title=\"struct wireguard_builder_rs::schema::vpn_ip_addresses::columns::vpn_network_id\">vpn_network_id</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_ip_addresses/struct.table.html\" title=\"struct wireguard_builder_rs::schema::vpn_ip_addresses::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::vpn_ip_addresses::columns::vpn_network_id"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_ip_addresses/columns/struct.ip_address.html\" title=\"struct wireguard_builder_rs::schema::vpn_ip_addresses::columns::ip_address\">ip_address</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_ip_addresses/struct.table.html\" title=\"struct wireguard_builder_rs::schema::vpn_ip_addresses::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::vpn_ip_addresses::columns::ip_address"]},{"text":"impl&lt;DB:&nbsp;Backend&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_networks/columns/struct.star.html\" title=\"struct wireguard_builder_rs::schema::vpn_networks::columns::star\">star</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_networks/struct.table.html\" title=\"struct wireguard_builder_rs::schema::vpn_networks::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::vpn_networks::columns::star"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_networks/columns/struct.id.html\" title=\"struct wireguard_builder_rs::schema::vpn_networks::columns::id\">id</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_networks/struct.table.html\" title=\"struct wireguard_builder_rs::schema::vpn_networks::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::vpn_networks::columns::id"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_networks/columns/struct.name.html\" title=\"struct wireguard_builder_rs::schema::vpn_networks::columns::name\">name</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_networks/struct.table.html\" title=\"struct wireguard_builder_rs::schema::vpn_networks::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::vpn_networks::columns::name"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_networks/columns/struct.description.html\" title=\"struct wireguard_builder_rs::schema::vpn_networks::columns::description\">description</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_networks/struct.table.html\" title=\"struct wireguard_builder_rs::schema::vpn_networks::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::vpn_networks::columns::description"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_networks/columns/struct.ip_network.html\" title=\"struct wireguard_builder_rs::schema::vpn_networks::columns::ip_network\">ip_network</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_networks/struct.table.html\" title=\"struct wireguard_builder_rs::schema::vpn_networks::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::vpn_networks::columns::ip_network"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_networks/columns/struct.subnetmask.html\" title=\"struct wireguard_builder_rs::schema::vpn_networks::columns::subnetmask\">subnetmask</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_networks/struct.table.html\" title=\"struct wireguard_builder_rs::schema::vpn_networks::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::vpn_networks::columns::subnetmask"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_networks/columns/struct.listen_port.html\" title=\"struct wireguard_builder_rs::schema::vpn_networks::columns::listen_port\">listen_port</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_networks/struct.table.html\" title=\"struct wireguard_builder_rs::schema::vpn_networks::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::vpn_networks::columns::listen_port"]},{"text":"impl&lt;DB&gt; QueryFragment&lt;DB&gt; for <a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_networks/columns/struct.interface_name.html\" title=\"struct wireguard_builder_rs::schema::vpn_networks::columns::interface_name\">interface_name</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Backend,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;<a class=\"struct\" href=\"wireguard_builder_rs/schema/vpn_networks/struct.table.html\" title=\"struct wireguard_builder_rs::schema::vpn_networks::table\">table</a> as QuerySource&gt;::FromClause: QueryFragment&lt;DB&gt;,&nbsp;</span>","synthetic":false,"types":["wireguard_builder_rs::schema::vpn_networks::columns::interface_name"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()