select product.* 
from product_with_extras as product
where product.id = $1 and product.catalog_id = $2