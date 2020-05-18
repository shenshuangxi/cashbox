import 'package:app/generated/i18n.dart';
import 'package:app/model/chain.dart';
import 'package:app/model/digit.dart';
import 'package:app/model/wallet.dart';
import 'package:app/res/resources.dart';
import 'package:app/util/log_util.dart';
import 'package:app/widgets/my_separator_line.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_easyrefresh/ball_pulse_footer.dart';
import 'package:flutter_easyrefresh/easy_refresh.dart';
import 'package:flutter_screenutil/flutter_screenutil.dart';

class SearchDigitPage extends StatefulWidget {
  @override
  _SearchDigitPageState createState() => _SearchDigitPageState();
}

class _SearchDigitPageState extends State<SearchDigitPage> {
  List<Digit> displayDigitsList = [];
  Wallet nowWalletM;
  Chain nowChain;
  TextEditingController _searchContentController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _searchContentController.text = "";
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      width: ScreenUtil().setWidth(90),
      height: ScreenUtil().setHeight(120),
      child: Scaffold(
        backgroundColor: Colors.transparent,
        appBar: AppBar(
            title: buildSearchInputWidget(),
            backgroundColor: Colors.transparent,
            elevation: 0,
            brightness: Brightness.light,
            actions: <Widget>[
              buildCancelWidget(),
            ]),
        body: Container(
            decoration: BoxDecoration(
              image: DecorationImage(image: AssetImage("assets/images/bg_graduate.png"), fit: BoxFit.fill),
            ),
            child: Column(
              children: <Widget>[
                Gaps.scaleVGap(5),
                buildDigitListAreaWidgets(),
              ],
            )),
      ),
    );
  }

  Widget buildCancelWidget() {
    return Container(
        width: ScreenUtil.instance.setWidth(10),
        child: Row(
          children: <Widget>[
            RichText(
              text: TextSpan(children: <TextSpan>[
                TextSpan(
                    text: S.of(context).cancel,
                    style: TextStyle(
                      decoration: TextDecoration.none,
                      color: Colors.white70,
                      fontSize: ScreenUtil.instance.setSp(3),
                      fontStyle: FontStyle.normal,
                    ),
                    recognizer: TapGestureRecognizer()
                      ..onTap = () {
                        if (_searchContentController.text == "") {
                          Navigator.pop(context);
                          return;
                        }
                        _searchContentController.text = "";
                      }),
              ]),
            ),
          ],
        ));
  }

  Widget buildSearchInputWidget() {
    return Container(
      width: ScreenUtil.instance.setWidth(60),
      //修饰黑色背景与圆角
      decoration: new BoxDecoration(
        border: Border.all(color: Colors.grey, width: ScreenUtil.instance.setWidth(1)), //灰色的一层边框
        color: Colors.grey,
        borderRadius: new BorderRadius.all(new Radius.circular(ScreenUtil.instance.setWidth(3))),
      ),
      alignment: Alignment.center,
      height: ScreenUtil.instance.setHeight(8),
      child: buildSearchTextFieldWidget(),
    );
  }

  Widget buildSearchTextFieldWidget() {
    return Container(
        child: TextField(
      cursorColor: Colors.white,
      //设置光标
      decoration: InputDecoration(
        contentPadding: new EdgeInsets.only(bottom: ScreenUtil.instance.setHeight(2.5)),
        border: InputBorder.none,
        icon: IconButton(
            icon: ImageIcon(
              AssetImage(
                "assets/images/ic_search.png",
              ),
            ),
            onPressed: () {
              _searchDigit(_searchContentController.text);
            }),
        hintText: "请输入代币名称或合约地址",
        hintStyle: new TextStyle(fontSize: ScreenUtil.instance.setSp(3), color: Colors.white),
      ),
      onSubmitted: (value) {
        _searchDigit(value);
      },
      controller: _searchContentController,
      //文本对齐方式(即光标初始位置)
      textAlign: TextAlign.start,
      style: new TextStyle(fontSize: ScreenUtil.instance.setSp(3), color: Colors.white),
    ));
  }

  Widget buildDigitListAreaWidgets() {
    return Container(
        height: ScreenUtil().setHeight(70),
        width: ScreenUtil().setWidth(90),
        child: Container(
          padding: EdgeInsets.only(left: ScreenUtil().setWidth(3), right: ScreenUtil().setWidth(3)),
          child: _digitListWidgets(),
        ));
  }

  //代币列表layout
  Widget _digitListWidgets() {
    return EasyRefresh.custom(
      footer: BallPulseFooter(),
      slivers: <Widget>[
        SliverList(
          delegate: SliverChildBuilderDelegate(
            (context, index) {
              return Container(
                child: _makeDigitListItem(index),
              );
            },
            childCount: displayDigitsList.length,
          ),
        ),
      ],
      onLoad: () async {
        //代币列表栏，下拉 刷新||加载 数据。
        await Future.delayed(
          Duration(seconds: 2),
          () {
            setState(() {
              this.displayDigitsList = displayDigitsList;
            });
          },
        );
      },
    );
  }

  //每个代币的layout
  Widget _makeDigitListItem(index) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: <Widget>[
        Container(
          alignment: Alignment.center,
          height: ScreenUtil().setHeight(17),
          margin: EdgeInsets.only(
            left: ScreenUtil().setWidth(3),
            right: ScreenUtil().setWidth(3),
          ),
          child: GestureDetector(
            onTap: () {
              try {
                // todo 保存 或者 更改显示状态
              } catch (e) {
                print("digit_list_page点击传值出现位置错误===>" + e.toString());
                LogUtil.e("digit_list_page", e.toString());
              }
            },
            child: Row(
              children: <Widget>[
                Container(
                  child: Image.asset("assets/images/ic_eth.png"),
                ),
                Container(
                  color: Colors.transparent,
                  padding: EdgeInsets.only(
                    top: ScreenUtil().setHeight(3),
                    left: ScreenUtil().setHeight(3),
                  ),
                  width: ScreenUtil().setWidth(30),
                  height: ScreenUtil().setHeight(10),
                  child: Text(
                    displayDigitsList[index].shortName ?? "",
                    style: TextStyle(
                      color: Colors.white,
                      fontSize: ScreenUtil.instance.setSp(3.5),
                    ),
                  ),
                ),
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
  }

  _searchDigit(String param) {
    print("onSubmitted is ===>" + param);
    if (param == null || param.isEmpty) {
      return false;
    }
    // todo 执行查找接口
    //Wallets.instance.queryNativeDigitListRecord(param);
    setState(() {
      //this.displayDigitsList = [];
    });
  }
}
