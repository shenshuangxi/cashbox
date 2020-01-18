import 'package:app/generated/i18n.dart';
import 'package:app/global_config/global_config.dart';
import 'package:app/model/digit.dart';
import 'package:app/model/rate.dart';
import 'package:app/model/tx_model/eth_transaction_model.dart';
import 'package:app/net/net_util.dart';
import 'package:app/routers/fluro_navigator.dart';
import 'package:app/routers/routers.dart';
import 'package:app/widgets/app_bar.dart';
import 'package:app/widgets/my_separator_line.dart';
import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_easyrefresh/ball_pulse_footer.dart';
import 'package:flutter_easyrefresh/easy_refresh.dart';
import 'package:flutter_screenutil/flutter_screenutil.dart';
import '../../res/resources.dart';

class TransactionHistoryPage extends StatefulWidget {
  @override
  _TransactionHistoryPageState createState() => _TransactionHistoryPageState();
}

class _TransactionHistoryPageState extends State<TransactionHistoryPage> {
  Future future;
  List<Digit> walletDataList = [];
  List<Digit> showDataList = [];

  @override
  void initState() {
    super.initState();
    future = getData(); //todo initData
  }

  Future<List<Digit>> getData() async {
    /*request(assembleEthBalanceUrl("0xa4512ca7618d8d12a30C28979153aB09809ED7fD")).then((res) {
      print("Eth_Balance=====================>" + res.toString());
      if (res != null && (res as Map).containsKey("result")) {
        print("Eth_Balance res.result.=====================>" + res["result"].toString());
      }
    }).catchError((e) {
      print("Eth_Balance e=====================>" + e.toString());
    });

    request(assembleErc20BalanceUrl("0xa4512ca7618d8d12a30C28979153aB09809ED7fD")).then((res) {
      print("Erc20_Balance=====================>" + res.toString());
      if (res != null && (res as Map).containsKey("result")) {
        print("Erc20_Balance res.result.=====================>" + res["result"].toString());
      }
    }).catchError((e) {
      print("Erc20_Balance e=====================>" + e.toString());
    });*/

    request(assembleEthTxListUrl("0xa4512ca7618d8d12a30C28979153aB09809ED7fD")).then((res) {
      print("Eth_Tx_List=====================>" + res.toString());
      // && (res as Map).containsKey("status") && res["status"] == 1
      var modelArray = [];
      if (res != null && (res as Map).containsKey("result")) {
        print("Eth_Tx_List res.result.=====================>" + res["result"].length.toString());
        for (var i = 0; i < res["result"].length; i++) {
          var ethTxModel = new EthTransactionModel();
          new EthTransactionModel()
            ..blockNumber = (res["result"][i]["blockNumber"])
            ..blockNumber = res["result"][i]["blockNumber"]
            ..timeStamp = res["result"][i]["timeStamp"]
            ..hash = res["result"][i]["hash"]
            ..nonce = res["result"][i]["nonce"]
            ..blockHash = res["result"][i]["blockHash"]
            ..transactionIndex = res["result"][i]["transactionIndex"]
            ..from = res["result"][i]["from"]
            ..to = res["result"][i]["to"]
            ..value = res["result"][i]["value"]
            ..gas = res["result"][i]["gas"]
            ..gasPrice = res["result"][i]["gasPrice"]
            ..isError = res["result"][i]["isError"]
            ..txreceipt_status = res["result"][i]["txreceipt_status"]
            ..input = res["result"][i]["input"]
            ..contractAddress = res["result"][i]["contractAddress"]
            ..cumulativeGasUsed = res["result"][i]["cumulativeGasUsed"]
            ..gasUsed = res["result"][i]["gasUsed"]
            ..confirmations = res["result"][i]["confirmations"];
          modelArray.add(ethTxModel);
        }
      }
      print("modelArray.length.toString()====>" + modelArray.length.toString());
    }).catchError((e) {
      print("Eth_Tx_List e=====================>" + e.toString());
    });

    /*request(assembleErc20TxListUrl("0xa4512ca7618d8d12a30C28979153aB09809ED7fD")).then((res) {
      print("Erc20_Tx_List=====================>" + res.toString());
      // && (res as Map).containsKey("status") && res["status"] == 1
      if (res != null && (res as Map).containsKey("result")) {
        print("Erc20_Tx_List res.result.=====================>" + res["result"].length.toString());
      }
    }).catchError((e) {
      print("Erc20_Tx_List e=====================>" + e.toString());
    });*/

    //todo mock data to test
    for (var i = 0; i < 10; i++) {
      Digit digit = EeeDigit();
      digit.chainId = "chainId001";
      digit.shortName = "ETH" + i.toString();
      digit.fullName = "ETHereum";
      digit.balance = "15";
      digit.money = "666";
      var digitRate = DigitRate();
      digitRate.volume = 0.035;
      digitRate.changeHourly = 0.096;
      digit.digitRate = digitRate;
      showDataList.add(digit);
    }
    return showDataList;
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      width: ScreenUtil().setWidth(90),
      height: ScreenUtil().setHeight(120),
      child: Scaffold(
        backgroundColor: Colors.transparent,
        appBar: MyAppBar(
          centerTitle: S.of(context).transaction_history,
          backgroundColor: Colors.transparent,
        ),
        body: Container(
          child: _buildTxHistoryWidget(),
        ),
      ),
    );
  }

  Widget _buildTxHistoryWidget() {
    return Container(
      width: ScreenUtil().setWidth(90),
      child: Column(
        children: <Widget>[
          Gaps.scaleVGap(1),
          _buildDigitBalanceWidget(),
          Gaps.scaleVGap(7),
          _buildDigitTxTitleWidget(),
          Gaps.scaleVGap(5),
          _buildDigitTxHistoryWidget(),
        ],
      ),
    );
  }

  Widget _buildDigitBalanceWidget() {
    return Container(
      color: Color.fromRGBO(101, 98, 98, 0.12),
      width: ScreenUtil().setWidth(90),
      height: ScreenUtil().setHeight(20.5),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.start,
        children: <Widget>[
          Gaps.scaleHGap(3),
          Container(
            alignment: Alignment.centerRight,
            width: ScreenUtil().setWidth(25),
            height: ScreenUtil().setHeight(8),
            child: Text(
              "6462314.0",
              textAlign: TextAlign.end,
              style: TextStyle(
                fontSize: ScreenUtil.instance.setSp(4),
                color: Colors.white,
              ),
              overflow: TextOverflow.ellipsis,
              maxLines: 1,
            ),
          ),
          Gaps.scaleHGap(0.5),
          Container(
            width: ScreenUtil().setWidth(8),
            child: Text(
              "ETH",
              textAlign: TextAlign.end,
              style: TextStyle(
                fontSize: ScreenUtil.instance.setSp(3.5),
                color: Colors.white,
              ),
              overflow: TextOverflow.ellipsis,
              maxLines: 1,
            ),
          ),
          Gaps.scaleHGap(0.5),
          Container(
            width: ScreenUtil().setWidth(15),
            child: Text(
              "≈" + "\$" + "6300.111311111",
              textAlign: TextAlign.center,
              style: TextStyle(
                fontSize: ScreenUtil.instance.setSp(3),
                color: Colors.lightBlueAccent,
              ),
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
            ),
          ),
          Gaps.scaleHGap(10),
          Container(
            //height: ScreenUtil().setHeight(8),
            child: FlatButton(
              color: Color.fromRGBO(26, 141, 198, 0.2),
              onPressed: () {
                NavigatorUtils.push(context, Routes.transferEthPage);
              },
              child: Text(
                S.of(context).transfer,
                style: TextStyle(
                  color: Colors.lightBlue,
                  fontSize: ScreenUtil.instance.setSp(3.5),
                ),
              ),
            ),
          ),
          Gaps.scaleHGap(6),
        ],
      ),
    );
  }

  Widget _buildDigitTxTitleWidget() {
    return Container(
        width: ScreenUtil().setWidth(90),
        child: Column(
          children: <Widget>[
            SingleChildScrollView(
              scrollDirection: Axis.horizontal,
              child: Row(
                mainAxisAlignment: MainAxisAlignment.start,
                children: <Widget>[
                  Gaps.scaleHGap(5),
                  Container(
                    width: ScreenUtil().setWidth(18),
                    child: Text(
                      S.of(context).transaction_history,
                      style: TextStyle(
                        color: Colors.white70,
                        fontSize: ScreenUtil.instance.setSp(3.5),
                      ),
                    ),
                  ),
                  Gaps.scaleHGap(45),
                  Container(
                    child: Text(
                      "2018.07",
                      style: TextStyle(
                        color: Colors.white70,
                      ),
                      textAlign: TextAlign.end,
                    ),
                  ),
                  Gaps.scaleHGap(6),
                ],
              ),
            ),
          ],
        ));
  }

  Widget _buildDigitTxHistoryWidget() {
    return Container(
      width: ScreenUtil().setWidth(90),
      child: Column(
        children: <Widget>[
          Container(
            height: ScreenUtil().setHeight(100),
            padding: EdgeInsets.only(left: ScreenUtil().setWidth(5), right: ScreenUtil().setWidth(5)),
            child: FutureBuilder(
              future: future,
              builder: (context, snapshot) {
                if (snapshot.hasError) {
                  return Text(S.of(context).fail_to_load_data_hint);
                }
                if (snapshot.hasData) {
                  return Container(
                    child: _makeRefreshWidgets(snapshot),
                  );
                } else {
                  return Text(
                    S.of(context).no_tx_history,
                    style: TextStyle(
                      color: Colors.white,
                      fontSize: ScreenUtil.instance.setSp(4),
                    ),
                  );
                }
              },
            ),
          )
        ],
      ),
    );
  }

  Widget _makeRefreshWidgets(snapshot) {
    return EasyRefresh.custom(
      footer: BallPulseFooter(),
      slivers: <Widget>[
        SliverList(
          delegate: SliverChildBuilderDelegate(
            (context, index) {
              return Container(
                height: ScreenUtil().setHeight(16),
                child: _makeTxItemWidget(index),
              );
            },
            childCount: showDataList.length,
          ),
        ),
      ],
      onLoad: () async {
        await Future.delayed(Duration(seconds: 2), () {
          print("onLoad");
          setState(() {
            //todo add Data
          });
        });
      },
    );
  }

  Widget _makeTxItemWidget(index) {
    return Container(
      width: ScreenUtil().setHeight(90),
      alignment: Alignment.center,
      child: GestureDetector(
        onTap: () {
          print("click tap is " + index.toString());
          NavigatorUtils.push(context, Routes.transactionEeeDetailPage);
        },
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            Container(
              alignment: Alignment.center,
              width: ScreenUtil().setWidth(90),
              height: ScreenUtil().setWidth(6),
              child: SingleChildScrollView(
                scrollDirection: Axis.horizontal,
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: <Widget>[
                    Container(
                      width: ScreenUtil().setWidth(18),
                      alignment: Alignment.centerLeft,
                      child: Text(
                        showDataList[index].shortName,
                        style: TextStyle(
                          color: Colors.greenAccent,
                          fontSize: ScreenUtil.instance.setSp(3.5),
                        ),
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                      ),
                    ),
                    Gaps.scaleHGap(30),
                    Container(
                      width: ScreenUtil().setWidth(30),
                      child: Text(
                        "0xD235654678891316546516879",
                        style: TextStyle(
                          color: Color.fromRGBO(255, 255, 255, 0.7),
                          fontSize: ScreenUtil.instance.setSp(3),
                        ),
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                      ),
                    ),
                  ],
                ),
              ),
            ),
            Container(
              alignment: Alignment.center,
              width: ScreenUtil().setWidth(90),
              height: ScreenUtil().setWidth(5),
              child: SingleChildScrollView(
                scrollDirection: Axis.horizontal,
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: <Widget>[
                    Container(
                      width: ScreenUtil().setWidth(18),
                      alignment: Alignment.centerLeft,
                      child: Text(
                        "* 发送中",
                        style: TextStyle(
                          color: Colors.white,
                          fontSize: ScreenUtil.instance.setSp(2.5),
                        ),
                        textAlign: TextAlign.start,
                      ),
                    ),
                    Gaps.scaleHGap(30),
                    Container(
                      width: ScreenUtil().setWidth(30),
                      child: Text(
                        "2019.07.01",
                        style: TextStyle(
                          color: Colors.white,
                          fontSize: ScreenUtil.instance.setSp(2.5),
                        ),
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                        textAlign: TextAlign.end,
                      ),
                    ),
                  ],
                ),
              ),
            ),
            Gaps.scaleVGap(2),
            Container(
              alignment: Alignment.bottomLeft,
              width: ScreenUtil().setWidth(90),
              height: ScreenUtil().setHeight(0.1),
              child: CustomPaint(
                foregroundPainter: MySeparatorLine(
                  lineColor: Colors.blueAccent,
                  width: ScreenUtil().setWidth(90),
                ),
              ),
            )
          ],
        ),
      ),
    );
  }
}
