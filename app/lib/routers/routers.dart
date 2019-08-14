import 'package:fluro/fluro.dart';
import 'package:flutter/material.dart';
import '../routers/router_handler.dart';

class Routes {
  static String homePage = '/homepage';
  static String splashPage = '/splashpage';
  static String publicPage = '/publicpage';
  static String createWalletPage = '/createwalletpage';

  static void configureRoutes(Router router) {
    router.notFoundHandler = new Handler(
        handlerFunc: (BuildContext context, Map<String, List<String>> params) {
      print('ERROR====>ROUTE WAS NOT FONUND!!!');
    });

    router.define(homePage, handler: homePageHandler);
    router.define(splashPage, handler: splashPageHandler);
    router.define(publicPage, handler: publicPageHandler);
    router.define(createWalletPage, handler: createWalletPageHandler);
  }
}
