select catalog.*
from catalog_with_products as catalog
where catalog.id = $1

