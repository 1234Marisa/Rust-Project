// 搜索框
$("header>div:nth-child(2)>button").click(function(){
	var getWords=$(this).parent().find("input[type='text']").val();
	$("#gdaib-mylists>div").each(function(){
		$(this).hide();
		if(($(this).find("div:nth-child(2)").find("p").text()).search(getWords)!=-1){
			$(this).show();
		}
	})
	$("#gdaib-goodslists-title>span>input[type='checkbox']").prop("checked",false);
	allSum();
})
// 购物车数据
var shop_names=[
	"Java从入门到精通（第6版） 明日科技 清华大学出版社",
	"Java从入门到精通（第5版） 明日科技 清华大学出版社",
	"Java从入门到精通（第3版） 明日科技 清华大学出版社",
	"Java从入门到精通（第2版） 明日科技 清华大学出版社",
	"循序渐进Vue.js 3前端开发实战教你活用Vue.js3全家桶及周边框架",
	"循序渐进Vue.js 3前端开发实战教你活用Vue.js3全家桶及周边框架",
	"循序渐进Vue.js 3前端开发实战教你活用Vue.js3全家桶及周边框架"];
var shop_regulars=["包装：【单件】普通包装",
	"包装：【单件】普通包装",
	"包装：【单件】普通包装",
	"包装：【单件】普通包装",
	"包装：【单件】精美包装",
	"包装：【单件】精美包装",
	"包装：【单件】精美包装"];
var shop_size=["型号：8成新","型号：8成新","型号：8成新","型号：9成新","型号：9成新","型号：9成新","型号：9成新"];
var shop_price=[99.00,98.00,88.00,79.99,45.00,53.00,70.00];
var shop_num=[1,1,1,1,1,1,1,3];
// 导入购物车数据
for(var i=1;i<shop_names.length;i++){
	var sum=shop_price[i]*shop_num[i];
	var shop_data="<div>"+
				"<div><input type='checkbox'/></div>"+
				"<div>"+
					"<img src='../images/gwc/book"+i+".jpg'>"+
					"<p>"+shop_names[i]+"</p>"+
					"<span>"+shop_regulars[i]+"<br>"+shop_size[i]+"</span>"+
				"</div>"+
				"<div>￥"+shop_price[i]+"</div>"+
				"<div><button>-</button><span>"+shop_num[i]+"</span><button>+</button></div>"+
				"<div>￥"+sum+"</div>"+
				"<div><a>移入收藏夹</a><br><a>删除</a></div>"+
			"</div>";
	$("#gdaib-mylists").append(shop_data);
}
// 结算按钮
$("#gdaib-shop-head>div>span:last-child>button").click(function(){
	var selectedItems=$("#gdaib-mylists>div>div:nth-child(1)>input[type='checkbox']:checked").length;
	if(selectedItems==0){
		alert("请选择商品！");
	}else{
		$("#gdaib-mylists>div:visible>div:nth-child(1)>input[type='checkbox']:checked").each(function(){
			$(this).parent().parent().remove();
		})
		allSum();
		alert("结算成功！");
	}
})
// 增加/减少数量按钮
$("#gdaib-mylists>div>div:nth-child(4)>button:nth-child(1)").click(function(){
	var num=$(this).parent().find("span").text()-1;
	if(!num==0){
		$(this).parent().find("span").text(num);
	}
	allSum();
})
$("#gdaib-mylists>div>div:nth-child(4)>button:nth-child(3)").click(function(){
	var num=parseInt($(this).parent().find("span").text())+1;
	$(this).parent().find("span").text(num);
	allSum();
})
// 数量变化事件
$("#gdaib-mylists>div>div:nth-child(4)>span").on("DOMNodeInserted",function(){
	var num=parseFloat($(this).text());
	var price=parseFloat($(this).parent().parent().find("div:nth-child(3)").text().substr(1));
	var sum=num*price;
	$(this).parent().parent().find("div:nth-child(5)").text("￥"+sum);
})
// 购物车金额总计
function allSum(){
	var all=0.00;
	var num=0;
	$("#gdaib-mylists>div>div:nth-child(1)>input[type='checkbox']").each(function(){
		if($(this).is(":checked")){
			if($(this).is(":visible")){
				all+=parseFloat($(this).parent().parent().find("div:nth-child(5)").text().substr(1));
				num++;
			}
		}
	})
	$("#gdaib-shop-head>div>span:nth-child(2)").text(num+"件");
	$("#gdaib-shop-head>div>span:nth-child(4)").text("￥"+all.toFixed(2));
	var visibleItems=$("#gdaib-mylists>div:visible").length;
	if($("#gdaib-mylists>div:visible>div:nth-child(1)>input[type='checkbox']:checked").length==visibleItems){
		$("#gdaib-goodslists-title>span>input[type='checkbox']").prop("checked",true);
	}else{
		$("#gdaib-goodslists-title>span>input[type='checkbox']").prop("checked",false);
	}
	console.log(visibleItems);
}
// 多选框
$("#gdaib-goodslists-title>span>input[type='checkbox']").click(function(){
	if($(this).is(":checked")){
		$("#gdaib-mylists>div>div:nth-child(1)>input[type='checkbox']").each(function(){
			if($(this).is(":visible")){
				$(this).prop("checked",true);
			}
		})
	}else{
		$("#gdaib-mylists>div:visible>div:nth-child(1)>input[type='checkbox']").prop("checked",false);
	}
	allSum();
})
$("#gdaib-mylists>div>div:nth-child(1)>input[type='checkbox']").click(function(){
	allSum();
})
// 购物车操作
function backToStyle(){
	$("#gdaib-mylists>div>div:nth-child(6)>a").mouseover(function(){
		$(this).css({
			"color":"red",
			"cursor":"pointer",
			"text-decoration":"underline"
		});
	})
	$("#gdaib-mylists>div>div:nth-child(6)>a").mouseout(function(){
		$(this).css({
			"color":"#808080",
			"text-decoration":"none"
		})
	})
}
backToStyle();
$("#gdaib-mylists>div>div:nth-child(6)>a:nth-child(1)").click(function(){
	if($(this).text()=="移入收藏夹"){
		$(this).css({
			"color":"#8A2BE2",
			"font-weight":"bold",
			"text-decoration":"none"
		});
		$(this).off("mouseout").off("mouseover");
		alert("收藏成功！");
		$(this).text("取消收藏");
	}
	else{
		var cancelCollect=confirm("是否取消收藏？");
		if(cancelCollect){
			$(this).text("移入收藏夹");
			backToStyle();
			$(this).css({
				"color":"#808080",
				"text-decoration":"none",
				"font-weight":"500"
			})
		}
	}
})
$("#gdaib-mylists>div>div:nth-child(6)>a:nth-child(3)").click(function(){
	var deleteItem=confirm("是否删除此商品？");
	if(deleteItem){
		$(this).parent().parent().remove();
		allSum();
	}
})