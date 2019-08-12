import 'package:flutter/widgets.dart';
import '../wallet_manager/wallet_manager.dart';
import 'package:flutter/material.dart';
import 'package:app/page/home_page.dart';

class SplashPage extends StatefulWidget {
  @override
  _SplashPageState createState() => _SplashPageState();
}

class _SplashPageState extends State<SplashPage> {
  var allWalletList = List();

  @override
  void initState() {
    // TODO: implement initState
    super.initState();
    _initWallet();
  }

  void _initWallet() async {
    //var allWalletList = await WalletMgr.instance.loadAllWalletList();
  }

  @override
  Widget build(BuildContext context) {
    if (allWalletList.length > 0) {
      print("goHomePage=>");
      return HomePage();
    } else {
      return Material(
        child: Text("this is splashpage"),
      );
    }
  }
}