select catalog.*
from catalog_with_products as catalog
order by catalog.created_at desc
limit $1 offset $2