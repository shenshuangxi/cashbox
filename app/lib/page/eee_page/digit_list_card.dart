import 'package:flutter/material.dart';
import 'package:flutter_screenutil/flutter_screenutil.dart';
import 'package:flutter_easyrefresh/easy_refresh.dart';
import 'package:dio/dio.dart';
import 'package:flutter_easyrefresh/material_footer.dart';
import 'package:flutter_easyrefresh/bezier_bounce_footer.dart';
import 'package:flutter_easyrefresh/ball_pulse_footer.dart';
import '../../model/digit.dart';
import '../../widgets/my_separator_line.dart';

class DigitListCard extends StatefulWidget {
  @override
  _DigitListCardState createState() => _DigitListCardState();
}

class _DigitListCardState extends State<DigitListCard>
    with AutomaticKeepAliveClientMixin {
  Future future;
  List<Digit> refreshDataList = [];

  @override
  bool get wantKeepAlive => true;

  @override
  void initState() {
    super.initState();
    future = getData();
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      height: ScreenUtil().setHeight(78),
      width: ScreenUtil().setWidth(90),
      child: FutureBuilder(
        future: future,
        builder: (context, snapshot) {
          if (snapshot.hasError) {
            return Center(
              child: Text("数据加载出错了，请尝试重新加载!~"),
            );
          }
          if (snapshot.hasData) {
            return Container(
              child: _makeRefreshWidgets(snapshot),
            );
          } else {
            return Text("something wrong");
          }
        },
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
              return Column(
                children: <Widget>[
                  Container(
                    alignment: Alignment.center,
                    height: ScreenUtil().setHeight(17),
                    child: GestureDetector(
                      onTap: () {
                        print("click is " + index.toString());
                      },
                      child: Row(
                        children: <Widget>[
                          Container(
                              child: Padding(
                            padding: EdgeInsets.only(
                              left: ScreenUtil().setHeight(10),
                            ),
                            child: Image.asset("assets/images/ic_eth.png"),
                          )),
                          Container(
                              child: Padding(
                                  padding: EdgeInsets.only(
                                    left: ScreenUtil().setHeight(3),
                                  ),
                                  child: Column(
                                    mainAxisAlignment: MainAxisAlignment.center,
                                    children: <Widget>[
                                      Container(
                                        color: Colors.transparent,
                                        padding: EdgeInsets.only(
                                          top: ScreenUtil().setHeight(3),
                                        ),
                                        width: ScreenUtil().setWidth(65),
                                        height: ScreenUtil().setHeight(10),
                                        child: Stack(
                                          children: <Widget>[
                                            Align(
                                              alignment: new FractionalOffset(
                                                  0.0, 0.0),
                                              child: Text(
                                                refreshDataList[index]
                                                        .shortName +
                                                    "*" +
                                                    refreshDataList[index]
                                                        .balance,
                                                style: TextStyle(
                                                  color: Colors.white,
                                                  fontSize: 15,
                                                ),
                                              ),
                                            ),
                                            Align(
                                              alignment:
                                                  FractionalOffset.topRight,
                                              child: Text(
                                                "≈" +
                                                    refreshDataList[index]
                                                        .money,
                                                style: TextStyle(
                                                    color: Colors.white,
                                                    fontSize: 16),
                                              ),
                                            ),
                                          ],
                                        ),
                                      ),
                                      Container(
                                        padding: EdgeInsets.only(
                                          top: ScreenUtil().setHeight(1),
                                        ),
                                        color: Colors.transparent,
                                        width: ScreenUtil().setWidth(65),
                                        height: ScreenUtil().setHeight(7),
                                        alignment: Alignment.center,
                                        child: Stack(
                                          children: <Widget>[
                                            Row(
                                              children: <Widget>[
                                                Text(
                                                  "\$3217.986",
                                                  style: TextStyle(
                                                    color:
                                                        Colors.lightBlueAccent,
                                                    fontSize: 10,
                                                  ),
                                                ),
                                                Padding(
                                                  padding: EdgeInsets.only(
                                                      left: ScreenUtil()
                                                          .setWidth(2.5)),
                                                  child: Text(
                                                    "10.87%",
                                                    style: TextStyle(
                                                        color:
                                                            Colors.yellowAccent,
                                                        fontSize: 10),
                                                  ),
                                                )
                                              ],
                                            ),
                                            Align(
                                              alignment:
                                                  FractionalOffset.topRight,
                                              child: Text(
                                                "-100",
                                                style: TextStyle(
                                                    fontSize: 10,
                                                    color: Colors.greenAccent),
                                              ),
                                            ),
                                          ],
                                        ),
                                      ),
                                    ],
                                  ))),
                        ],
                      ),
                    ),
                  ),
                  Container(
                    alignment: Alignment.topLeft,
                    width: ScreenUtil().setWidth(75),
                    height: ScreenUtil().setHeight(0.1),
                    child: CustomPaint(
                      foregroundPainter: MySeparatorLine(
                        lineColor: Colors.blueAccent,
                        width: ScreenUtil().setWidth(75),
                      ),
                    ),
                  )
                ],
              );
            },
            childCount: refreshDataList.length,
          ),
        ),
      ],
      onLoad: () async {
        await Future.delayed(Duration(seconds: 2), () {
          setState(() {
            Digit digit = Digit("chainId008");
            for (var i = 0; i < 10; i++) {
              digit.shortName = "BTC";
              digit.fullName = "Bitcoin";
              digit.balance = "15";
              digit.money = "63.15";
              refreshDataList.add(digit);
            }
          });
        });
      },
    );
  }

  Future<List<Digit>> getData() async {
    //use mock data todo
    //walletManage.nowWallet.noChain.loadDigits()
    Digit digit = Digit("chainId001");
    for (var i = 0; i < 10; i++) {
      digit.shortName = "ETH";
      digit.fullName = "ETHereum";
      digit.balance = "15";
      digit.money = "666";
      refreshDataList.add(digit);
    }
    //todo mock data to test
    return refreshDataList;
  }
}