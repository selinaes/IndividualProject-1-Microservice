/* An actix microservice that has three routes
A. / that returns a hello page with moon image
B. /moon that returns the current moon phase
C. /moon?date=YYYY-MM-DD that returns the moon phase for the given date
*/

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
